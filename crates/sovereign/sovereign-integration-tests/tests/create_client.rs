#![recursion_limit = "256"]
use core::time::Duration;
use std::env::var;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use eyre::eyre;
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error;
use hermes_cosmos_test_components::chain::types::amount::Amount;
use hermes_cosmos_test_components::chain::types::proposal_status::ProposalStatus;
use hermes_cosmos_test_components::chain_driver::traits::proposal_status::CanPollProposalStatus;
use hermes_cosmos_test_components::chain_driver::traits::vote_proposal::CanVoteProposal;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::CanBuildConnectionHandshakeMessages;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::CanBuildConnectionHandshakePayloads;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientEvent;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_chain_components::sovereign::types::payloads::client::SovereignCreateClientOptions;
use hermes_sovereign_integration_tests::contexts::sovereign_bootstrap::SovereignBootstrap;
use hermes_sovereign_relayer::contexts::sovereign_chain::SovereignChain;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::proposal::deposit::CanDepositProposal;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_wasm_client_components::contexts::wasm_counterparty::WasmCounterparty;
use hermes_wasm_test_components::traits::upload_client_code::CanUploadWasmClientCode;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use serde_json::Value as JsonValue;
use sha2::{Digest, Sha256};
use tokio::runtime::Builder;
use toml::Value as TomlValue;
use tracing::info;

#[tracing::instrument]
#[test]
pub fn test_create_sovereign_client_on_cosmos() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix = format!(
        "{}-{}",
        SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis(),
        rand::random::<u64>()
    );

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    // TODO: load parameters from environment variables
    let bootstrap = Arc::new(CosmosBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: format!("./test-data/{store_postfix}/chains").into(),
        chain_command_path: "simd".into(),
        account_prefix: "sov".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(modify_wasm_client_genesis),
        comet_config_modifier: Box::new(modify_wasm_node_config),
    });

    let celestia_bootstrap = CelestiaBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        chain_store_dir: store_dir.join("chains"),
        bridge_store_dir: store_dir.join("bridges"),
    };

    let sovereign_bootstrap = SovereignBootstrap {
        runtime: runtime.clone(),
        rollup_store_dir: store_dir.join("rollups"),
        rollup_command_path: "node".into(),
        account_prefix: "sov".into(),
    };

    let create_client_settings = ClientSettings::Tendermint(Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: None,
        trust_threshold: TrustThreshold::ONE_THIRD,
    });

    let wasm_client_code_path = var("WASM_FILE_PATH").expect("Wasm file is required").into();

    let wasm_client_bytes = std::fs::read(&wasm_client_code_path)?;

    let wasm_code_hash: [u8; 32] = {
        let mut hasher = Sha256::new();
        hasher.update(wasm_client_bytes);
        hasher.finalize().into()
    };

    let sovereign_create_client_options = SovereignCreateClientOptions {
        code_hash: wasm_code_hash.into(),
    };

    tokio_runtime.block_on(async move {
        let cosmos_chain_driver = bootstrap.bootstrap_chain("cosmos-1").await?;

        let cosmos_chain = cosmos_chain_driver.chain();

        let celestia_chain_driver = celestia_bootstrap.bootstrap_chain("private").await?;

        let celestia_chain = celestia_chain_driver.chain();

        let bridge_driver = celestia_bootstrap
            .bootstrap_bridge(&celestia_chain_driver)
            .await?;

        let rollup_driver = sovereign_bootstrap
            .bootstrap_rollup(&celestia_chain_driver, &bridge_driver, "test-rollup")
            .await?;

        // Upload Wasm contract on Cosmos chain
        cosmos_chain_driver.upload_wasm_client_code(
            &wasm_client_code_path,
            "sovereign-client",
            "Sovereign Wasm Client",
            &cosmos_chain_driver.validator_wallet,
        ).await?;

        cosmos_chain_driver.poll_proposal_status(&1, &ProposalStatus::DepositPeriod).await?;

        cosmos_chain_driver.deposit_proposal(
            &1,
            &Amount::new(100000000, cosmos_chain_driver.genesis_config.staking_denom.clone()),
            &cosmos_chain_driver.validator_wallet
        ).await?;

        cosmos_chain_driver.poll_proposal_status(&1, &ProposalStatus::VotingPeriod).await?;

        cosmos_chain_driver.vote_proposal(&1, &cosmos_chain_driver.validator_wallet).await?;

        cosmos_chain_driver.poll_proposal_status(&1, &ProposalStatus::Passed).await?;

        let sovereign_chain = SovereignChain {
            runtime: runtime.clone(),
            data_chain: celestia_chain.clone(),
            rollup: rollup_driver.rollup,
        };

        // Create Sovereign client on Cosmos chain
        let create_client_payload = <SovereignChain as CanBuildCreateClientPayload<CosmosChain>>::build_create_client_payload(
            &sovereign_chain,
            &sovereign_create_client_options
        ).await?;

        let create_client_message = <CosmosChain as CanBuildCreateClientMessage<SovereignChain>>::build_create_client_message(
            cosmos_chain,
            create_client_payload,
        ).await?;

        let _events = cosmos_chain.send_message(create_client_message).await?;

        let wasm_client_id = ClientId::from_str("08-wasm-0").map_err(|e| eyre!("Failed to create a Client ID from string '08-wasm-0': {e}"))?;

        let sovereign_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<SovereignChain>>::query_client_state_with_latest_height(cosmos_chain, &wasm_client_id).await?;

        // Create Celestia client (DA client) on Cosmos chain
        let create_celestia_client_payload = <CosmosChain as CanBuildCreateClientPayload<CosmosChain>>::build_create_client_payload(
            celestia_chain,
            &create_client_settings
        ).await?;

        let create_celestia_client_message = <CosmosChain as CanBuildCreateClientMessage<CosmosChain>>::build_create_client_message(
            cosmos_chain,
            create_celestia_client_payload,
        ).await?;

        let events = cosmos_chain.send_message(create_celestia_client_message).await?;

        let create_client_event = events.into_iter()
            .find_map(<CosmosChain as HasCreateClientEvent<CosmosChain>>::try_extract_create_client_event)
            .ok_or_else(|| eyre!("Could not extract Celestia create client event"))?;

        let celestia_client_id = create_client_event.client_id;

        let wasm_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<WasmCounterparty>>::query_client_state_with_latest_height(cosmos_chain, &wasm_client_id).await?;

        let celestia_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<CosmosChain>>::query_client_state_with_latest_height(cosmos_chain, &celestia_client_id).await?;

        let dummy_trusted_height = RollupHeight { slot_number: wasm_client_state.latest_height.revision_height() as u64 };
        let dummy_target_height = RollupHeight { slot_number: (celestia_client_state.latest_height.revision_height()) as u64 };

        // Update Sovereign client state
        let update_client_payload = <SovereignChain as CanBuildUpdateClientPayload<CosmosChain>>::build_update_client_payload(
            &sovereign_chain,
            &dummy_trusted_height,
            &dummy_target_height,
            sovereign_client_state
        ).await?;

        let update_client_messages = <CosmosChain as CanBuildUpdateClientMessage<SovereignChain>>::build_update_client_message(
            cosmos_chain,
            &wasm_client_id,
            update_client_payload,
        ).await?;

        for update_message in update_client_messages.into_iter() {
            // TODO: remove assertion once the dummy data used to update client is replaced with correct data
            let events = cosmos_chain.send_message(update_message).await;
            assert!(events.is_err(), "Client update will fail due to next validator set hash validation failure");
        }

        let sovereign_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<SovereignChain>>::query_client_state_with_latest_height(cosmos_chain, &wasm_client_id).await?;

        let connection_init_payload = <SovereignChain as CanBuildConnectionHandshakePayloads<CosmosChain>>::build_connection_open_init_payload(&sovereign_chain, &sovereign_client_state).await?;

        let options = CosmosInitConnectionOptions {
            delay_period: Duration::from_secs(0),
            connection_version: Version::default(),
        };

        // Placeholder for Sovereign client ID
        let sovereign_client_id = ClientId::from_str("sovereign-1").unwrap();

        // Assert that the connection Init fails with an invalid client
        {
            let connection_init_payload = <SovereignChain as CanBuildConnectionHandshakePayloads<CosmosChain>>::build_connection_open_init_payload(&sovereign_chain, &sovereign_client_state).await?;

            let wrong_wasm_client_id = ClientId::from_str("08-wasm-12").map_err(|e| eyre!("Failed to create a Client ID from string '08-wasm-0': {e}"))?;

            let connection_init_message = <CosmosChain as CanBuildConnectionHandshakeMessages<SovereignChain>>::build_connection_open_init_message(cosmos_chain, &wrong_wasm_client_id, &sovereign_client_id, &options, connection_init_payload).await?;

            let connection_init_event = cosmos_chain.send_message(connection_init_message).await;

            assert!(connection_init_event.is_err());
        }

        let connection_init_message = <CosmosChain as CanBuildConnectionHandshakeMessages<SovereignChain>>::build_connection_open_init_message(cosmos_chain, &wasm_client_id, &sovereign_client_id, &options, connection_init_payload).await?;

        let connection_init_event = cosmos_chain.send_message(connection_init_message).await?;

        info!("{:#?}", connection_init_event);

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

fn modify_wasm_node_config(config: &mut TomlValue) -> Result<(), Error> {
    config
        .get_mut("rpc")
        .and_then(|rpc| rpc.as_table_mut())
        .ok_or_else(|| eyre!("Failed to retrieve `rpc` in app configuration"))?
        .insert(
            "max_body_bytes".to_string(),
            TomlValue::Integer(10001048576),
        );

    Ok(())
}

fn modify_wasm_client_genesis(genesis: &mut serde_json::Value) -> Result<(), Error> {
    let max_deposit_period = genesis
        .get_mut("app_state")
        .and_then(|app_state| app_state.get_mut("gov"))
        .and_then(|gov| gov.get_mut("params"))
        .and_then(|deposit_params| deposit_params.as_object_mut())
        .ok_or_else(|| eyre!("Failed to retrieve `deposit_params` in genesis configuration"))?;

    max_deposit_period
        .insert(
            "max_deposit_period".to_owned(),
            JsonValue::String("10s".to_owned()),
        )
        .ok_or_else(|| eyre!("Failed to update `max_deposit_period` in genesis configuration"))?;

    let voting_period = genesis
        .get_mut("app_state")
        .and_then(|app_state| app_state.get_mut("gov"))
        .and_then(|gov| gov.get_mut("params"))
        .and_then(|voting_params| voting_params.as_object_mut())
        .ok_or_else(|| eyre!("Failed to retrieve `voting_params` in genesis configuration"))?;

    voting_period
        .insert(
            "voting_period".to_owned(),
            serde_json::Value::String("10s".to_owned()),
        )
        .ok_or_else(|| eyre!("Failed to update `voting_period` in genesis configuration"))?;

    let allowed_clients = genesis
        .get_mut("app_state")
        .and_then(|app_state| app_state.get_mut("ibc"))
        .and_then(|ibc| ibc.get_mut("client_genesis"))
        .and_then(|client_genesis| client_genesis.get_mut("params"))
        .and_then(|params| params.get_mut("allowed_clients"))
        .and_then(|allowed_clients| allowed_clients.as_array_mut())
        .ok_or_else(|| eyre!("Failed to retrieve `allowed_clients` in genesis configuration"))?;

    allowed_clients.push(JsonValue::String("08-wasm".to_string()));

    Ok(())
}
