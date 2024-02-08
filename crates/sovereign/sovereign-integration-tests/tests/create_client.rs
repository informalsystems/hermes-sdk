#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;

use eyre::Error;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::components::message_sender::CanSendSingleMessage;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_cosmos_relayer::contexts::sovereign_chain::SovereignChain;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use tokio::runtime::Builder;

// #[test]
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
        account_prefix: "cosmos".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    });

    let create_client_settings = ClientSettings::Tendermint(Settings {
        max_clock_drift: Duration::from_secs(40),
        trusting_period: None,
        trust_threshold: TrustThreshold::ONE_THIRD,
    });

    tokio_runtime.block_on(async move {
        let cosmos_chain_driver = bootstrap.bootstrap_chain("cosmos-1").await?;

        let cosmos_chain = cosmos_chain_driver.chain();

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
