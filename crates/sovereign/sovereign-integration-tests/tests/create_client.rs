#![recursion_limit = "256"]
use eyre::eyre;
use tokio::time::sleep;

use core::time::Duration;
use hermes_cosmos_test_components::chain_driver::traits::deposit_proposal::CanDepositProposal;
use hermes_cosmos_test_components::chain_driver::traits::proposal_status::CanQueryGovernanceProposalStatus;
use hermes_cosmos_test_components::chain_driver::traits::vote_proposal::CanVoteProposal;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::send_message::CanSendSingleMessage;
use serde_json::Value as JsonValue;
use std::env::var;
use std::sync::Arc;
use toml::Value as TomlValue;

use eyre::Error;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_integration_tests::contexts::chain_driver::CosmosChainDriver;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::chain_driver::traits::store_wasm_client::CanUploadWasmClientCode;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_cosmos_relayer::contexts::sovereign_chain::SovereignChain;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use tokio::runtime::Builder;

#[test]
pub fn test_create_sovereign_client_on_cosmos() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    // TODO: load parameters from environment variables
    let bootstrap = Arc::new(CosmosBootstrap {
        runtime: runtime.clone(),
        builder,
        should_randomize_identifiers: true,
        chain_store_dir: "./test-data".into(),
        chain_command_path: "simd".into(),
        account_prefix: "sov".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(|genesis| {
            let max_deposit_period = genesis
                .get_mut("app_state")
                .and_then(|app_state| app_state.get_mut("gov"))
                .and_then(|gov| gov.get_mut("params"))
                .and_then(|deposit_params| deposit_params.as_object_mut())
                .ok_or_else(|| {
                    eyre!("Failed to retrieve `deposit_params` in genesis configuration")
                })?;

            max_deposit_period
                .insert(
                    "max_deposit_period".to_owned(),
                    JsonValue::String("10s".to_owned()),
                )
                .ok_or_else(|| {
                    eyre!("Failed to update `max_deposit_period` in genesis configuration")
                })?;

            let voting_period = genesis
                .get_mut("app_state")
                .and_then(|app_state| app_state.get_mut("gov"))
                .and_then(|gov| gov.get_mut("params"))
                .and_then(|voting_params| voting_params.as_object_mut())
                .ok_or_else(|| {
                    eyre!("Failed to retrieve `voting_params` in genesis configuration")
                })?;

            voting_period
                .insert(
                    "voting_period".to_owned(),
                    serde_json::Value::String("10s".to_owned()),
                )
                .ok_or_else(|| {
                    eyre!("Failed to update `voting_period` in genesis configuration")
                })?;

            let allowed_clients = genesis
                .get_mut("app_state")
                .and_then(|app_state| app_state.get_mut("ibc"))
                .and_then(|ibc| ibc.get_mut("client_genesis"))
                .and_then(|client_genesis| client_genesis.get_mut("params"))
                .and_then(|params| params.get_mut("allowed_clients"))
                .and_then(|allowed_clients| allowed_clients.as_array_mut())
                .ok_or_else(|| {
                    eyre!("Failed to retrieve `allowed_clients` in genesis configuration")
                })?;

            allowed_clients.push(JsonValue::String("08-wasm".to_string()));

            Ok(())
        }),
        comet_config_modifier: Box::new(|config| {
            config
                .get_mut("rpc")
                .and_then(|rpc| rpc.as_table_mut())
                .ok_or_else(|| eyre!("Failed to retrieve `rpc` in app configuration"))?
                .insert(
                    "max_body_bytes".to_string(),
                    TomlValue::Integer(10001048576),
                );
            Ok(())
        }),
    });

    let create_client_settings = ClientSettings::Tendermint(Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: None,
        trust_threshold: TrustThreshold::ONE_THIRD,
    });

    let wasm_client_code_path = var("WASM_FILE_PATH")
        .unwrap_or_else(|_| "tests/utils/sov_celestia_client_cw.wasm".to_string())
        .into();

    tokio_runtime.block_on(async move {
        let cosmos_chain_driver = bootstrap.bootstrap_chain("cosmos-1").await?;

        let cosmos_chain = cosmos_chain_driver.chain();

        cosmos_chain_driver.store_wasm_client_code(
            &wasm_client_code_path,
            "tmp",
            "tmp",
            "validator",
        ).await?;

        assert_eventual_governance_status(&cosmos_chain_driver, "1", "PROPOSAL_STATUS_DEPOSIT_PERIOD").await?;

        cosmos_chain_driver.deposit_proposal("1", "100000000stake", "validator").await?;

        assert_eventual_governance_status(&cosmos_chain_driver, "1", "PROPOSAL_STATUS_VOTING_PERIOD").await?;

        cosmos_chain_driver.vote_proposal("1", "validator").await?;

        assert_eventual_governance_status(&cosmos_chain_driver, "1", "PROPOSAL_STATUS_PASSED").await?;

        let sovereign_chain = SovereignChain {
            runtime: runtime.clone(),
        };

        let create_client_payload = <SovereignChain as CanBuildCreateClientPayload<CosmosChain>>::build_create_client_payload(
            &sovereign_chain,
            &create_client_settings
        ).await?;

        let create_client_message = <CosmosChain as CanBuildCreateClientMessage<SovereignChain>>::build_create_client_message(
            cosmos_chain,
            create_client_payload,
        ).await?;

        let _events = cosmos_chain.send_message(create_client_message).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

async fn assert_eventual_governance_status(
    cosmos_chain_driver: &CosmosChainDriver,
    governance_id: &str,
    expected_status: &str,
) -> Result<(), Error> {
    for _ in 0..15 {
        let exec_output = cosmos_chain_driver
            .query_proposal_status(governance_id)
            .await?;
        if exec_output == expected_status {
            return Ok(());
        } else {
            sleep(Duration::from_secs(1)).await;
        }
    }
    Err(eyre!(
        "Governance proposal `{governance_id}` was not in status `{expected_status}`"
    ))
}
