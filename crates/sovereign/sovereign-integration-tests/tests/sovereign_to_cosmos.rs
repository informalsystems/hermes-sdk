#![recursion_limit = "256"]
use core::time::Duration;
use std::env::var;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use eyre::eyre;
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::CanBuildChannelHandshakeMessages;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::CanBuildConnectionHandshakeMessages;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::CanBuildConnectionHandshakePayloads;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::HasChannelOpenInitEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::connection::HasConnectionOpenInitEvent;
use hermes_relayer_components::relay::traits::client_creator::CanCreateClient;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_chain_components::sovereign::types::payloads::client::SovereignCreateClientOptions;
use hermes_sovereign_integration_tests::contexts::cosmos_bootstrap::CosmosWithWasmClientBootstrap;
use hermes_sovereign_integration_tests::contexts::sovereign_bootstrap::SovereignBootstrap;
use hermes_sovereign_relayer::contexts::cosmos_to_sovereign_relay::CosmosToSovereignRelay;
use hermes_sovereign_relayer::contexts::sovereign_chain::SovereignChain;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_wasm_client_components::contexts::wasm_counterparty::WasmCounterparty;
use ibc::core::client::types::Height;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::config::types::TrustThreshold;
use ibc_relayer_types::core::ics03_connection::version::Version;
use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::core::ics04_channel::version::Version as ChannelVersion;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, PortId};
use sha2::{Digest, Sha256};
use sov_celestia_client::types::client_state::test_util::TendermintParamsConfig;
use sov_celestia_client::types::sovereign::SovereignParamsConfig;
use tokio::runtime::Builder;
use tokio::time::sleep;
use tracing::info;

#[tracing::instrument]
#[test]
pub fn test_sovereign_to_cosmos() -> Result<(), Error> {
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
    let node_binary = var("ROLLUP_PATH")
        .unwrap_or_else(|_| "node".to_string())
        .into();

    let wasm_client_code_path =
        PathBuf::from(var("WASM_FILE_PATH").expect("Wasm file is required"));

    // TODO: load parameters from environment variables
    let cosmos_bootstrap = Arc::new(CosmosWithWasmClientBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: format!("./test-data/{store_postfix}/chains").into(),
        chain_command_path: "simd".into(),
        account_prefix: "sov".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        wasm_client_code_path: wasm_client_code_path.clone(),
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
        rollup_command_path: node_binary,
        account_prefix: "sov".into(),
    };

    let wasm_client_bytes = std::fs::read(&wasm_client_code_path)?;

    let wasm_code_hash: [u8; 32] = {
        let mut hasher = Sha256::new();
        hasher.update(wasm_client_bytes);
        hasher.finalize().into()
    };

    tokio_runtime.block_on(async move {
        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos-1").await?;

        let cosmos_chain = cosmos_chain_driver.chain();

        let celestia_chain_id = "private";

        let celestia_chain_driver = celestia_bootstrap.bootstrap_chain(celestia_chain_id).await?;

        let celestia_chain = celestia_chain_driver.chain();

        let bridge_driver = celestia_bootstrap
            .bootstrap_bridge(&celestia_chain_driver)
            .await?;

        let rollup_id = "test-rollup";

        let rollup_driver = sovereign_bootstrap
            .bootstrap_rollup(&celestia_chain_driver, &bridge_driver, rollup_id)
            .await?;

        let rollup = rollup_driver.rollup;

        let sovereign_chain = SovereignChain {
            runtime: runtime.clone(),
            data_chain: celestia_chain.clone(),
            rollup: rollup.clone(),
        };

        let rollup_genesis_da_height = Height::new(0, rollup_driver.node_config.runner.genesis_height)?;

        let sovereign_params = SovereignParamsConfig::builder()
            .genesis_da_height(rollup_genesis_da_height)
            .latest_height(Height::min(0)) // dummy value; overwritten by rollup latest height while creating client payload
            .build();

        let tendermint_params = TendermintParamsConfig::builder().chain_id(celestia_chain_id.parse()?).build();

        let sovereign_create_client_options = SovereignCreateClientOptions {
            tendermint_params_config: tendermint_params,
            sovereign_client_params: sovereign_params,
            code_hash: wasm_code_hash.into(),
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

        let wasm_client_id = ClientId::from_str("08-wasm-0")?;

        let sovereign_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<SovereignChain>>::query_client_state_with_latest_height(cosmos_chain, &wasm_client_id).await?;

        let wasm_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<WasmCounterparty>>::query_client_state_with_latest_height(cosmos_chain, &wasm_client_id).await?;

        let trusted_height = RollupHeight { slot_number: wasm_client_state.latest_height.revision_height() };

        // Wait for the rollup to progress before we build the update client for the next height
        sleep(Duration::from_secs(1)).await;

        let rollup_target_height = rollup.query_chain_height().await?;

        info!("Latest rollup height: {:?}", rollup_target_height);

        // TODO(rano): remove this sleep.
        sleep(Duration::from_secs(2)).await;

        // Update Sovereign client state
        let update_client_payload = <SovereignChain as CanBuildUpdateClientPayload<CosmosChain>>::build_update_client_payload(
            &sovereign_chain,
            &trusted_height,
            &rollup_target_height,
            sovereign_client_state
        ).await?;

        let update_client_messages = <CosmosChain as CanBuildUpdateClientMessage<SovereignChain>>::build_update_client_message(
            cosmos_chain,
            &wasm_client_id,
            update_client_payload,
        ).await?;

        for update_message in update_client_messages.into_iter() {
            cosmos_chain.send_message(update_message).await?;
        }

        let sovereign_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<SovereignChain>>::query_client_state_with_latest_height(cosmos_chain, &wasm_client_id).await?;

        // Cosmos client on Sovereign rollup
        let create_client_settings = ClientSettings::Tendermint(Settings {
            max_clock_drift: Duration::from_secs(40),
            trusting_period: None,
            trust_threshold: TrustThreshold::ONE_THIRD,
        });

        let sovereign_client_id = CosmosToSovereignRelay::create_client(
            DestinationTarget,
            &sovereign_chain,
            cosmos_chain,
            &create_client_settings,
        )
        .await?;

        info!("client ID of Cosmos on Sovereign: {:?}", sovereign_client_id);

        let connection_init_payload = <SovereignChain as CanBuildConnectionHandshakePayloads<CosmosChain>>::build_connection_open_init_payload(&sovereign_chain, &sovereign_client_state).await?;

        let options = CosmosInitConnectionOptions {
            delay_period: Duration::from_secs(0),
            connection_version: Version::default(),
        };

        // Assert that the connection Init fails with an invalid client
        {
            let connection_init_payload = <SovereignChain as CanBuildConnectionHandshakePayloads<CosmosChain>>::build_connection_open_init_payload(&sovereign_chain, &sovereign_client_state).await?;

            let wrong_wasm_client_id = ClientId::from_str("08-wasm-12").map_err(|e| eyre!("Failed to create a Client ID from string '08-wasm-0': {e}"))?;

            let connection_init_message = <CosmosChain as CanBuildConnectionHandshakeMessages<SovereignChain>>::build_connection_open_init_message(cosmos_chain, &wrong_wasm_client_id, &sovereign_client_id, &options, connection_init_payload).await?;

            let connection_init_event = cosmos_chain.send_message(connection_init_message).await;

            assert!(connection_init_event.is_err());
        }

        let connection_init_message = <CosmosChain as CanBuildConnectionHandshakeMessages<SovereignChain>>::build_connection_open_init_message(cosmos_chain, &wasm_client_id, &sovereign_client_id, &options, connection_init_payload).await?;

        let events = cosmos_chain.send_message(connection_init_message).await?;

        info!("Connection Open Init events: {:#?}", events);

        let connection_init_event = events.into_iter()
            .find_map(<CosmosChain as HasConnectionOpenInitEvent<CosmosChain>>::try_extract_connection_open_init_event)
            .ok_or_else(|| eyre!("Could not extract Celestia create client event"))?;

        let connection_id = connection_init_event.connection_id;

        let _cosmos_client_state = <SovereignChain as CanQueryClientStateWithLatestHeight<CosmosChain>>::query_client_state_with_latest_height(&sovereign_chain, &sovereign_client_id).await?;

        let _cosmos_height = <CosmosChain as CanQueryChainHeight>::query_chain_height(cosmos_chain).await?;

        // // TODO(rano): fails as it expects a Tendermint client; but Sovereign client is a Wasm proxy client.
        // let connection_try_payload = <CosmosChain as CanBuildConnectionHandshakePayloads<SovereignChain>>::build_connection_open_try_payload(cosmos_chain, &cosmos_client_state, &cosmos_height, &wasm_client_id, &connection_id).await?;

        // let connection_try_message = <SovereignChain as CanBuildConnectionHandshakeMessages<CosmosChain>>::build_connection_open_try_message(&sovereign_chain, &sovereign_client_id, &wasm_client_id, &connection_id, connection_try_payload).await?;
        // let connection_try_event = sovereign_chain.send_message(connection_try_message).await?;

        // info!("ConnectionTry event at Sovereign: {:#?}", connection_try_event);


        let options: CosmosInitChannelOptions = CosmosInitChannelOptions {
            ordering: Ordering::Unordered,
            connection_hops: vec!(connection_id),
            channel_version: ChannelVersion::default(),
        };

        let port_id = PortId::transfer();
        let counterparty_port_id = PortId::transfer();

        let channel_init_message = <CosmosChain as CanBuildChannelHandshakeMessages<SovereignChain>>::build_channel_open_init_message(cosmos_chain, &port_id, &counterparty_port_id, &options).await?;

        let events = cosmos_chain.send_message(channel_init_message).await?;

        let channel_init_event = events.into_iter()
            .find_map(<CosmosChain as HasChannelOpenInitEvent<CosmosChain>>::try_extract_channel_open_init_event)
            .ok_or_else(|| eyre!("Could not extract Celestia create client event"))?;

        let channel_id = channel_init_event.channel_id;

        info!("Retrieved Channel ID from Channel Open Init event: {channel_id}");

        // let rollup_target_height = rollup.query_chain_height().await?;

        // let sovereign_client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<SovereignChain>>::query_client_state_with_latest_height(cosmos_chain, &wasm_client_id).await?;

        // let channel_ack_payload = <SovereignChain as CanBuildChannelHandshakePayloads<CosmosChain>>::build_channel_open_ack_payload(&sovereign_chain, &sovereign_client_state, &rollup_target_height, &port_id, &channel_id).await?;

        // TODO replace with channel ID retrieved from Channel Open Try event
        // let counterparty_channel_id = ChannelId::from_str("placeholder-id").unwrap();

        // let channel_ack_message = <CosmosChain as CanBuildChannelHandshakeMessages<SovereignChain>>::build_channel_open_ack_message(cosmos_chain, &port_id, &channel_id, &counterparty_channel_id, channel_ack_payload).await?;

        // let events = cosmos_chain.send_message(channel_ack_message).await?;

        // info!("Channel Open Ack events: {events:#?}");

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
