#![recursion_limit = "256"]

use core::time::Duration;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_cosmos_relayer::contexts::sovereign_chain::SovereignChain;
use hermes_sovereign_integration_tests::contexts::bootstrap::SovereignBootstrap;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::core::ics02_client::trust_threshold::TrustThreshold;
use tokio::runtime::Builder;

#[test]
fn test_sovereign_ibc() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    let cosmos_bootstrap = Arc::new(CosmosBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: true,
        chain_store_dir: store_dir.join("chains"),
        chain_command_path: "simd".into(),
        account_prefix: "cosmos".into(),
        staking_denom: "stake".into(),
        transfer_denom: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
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

    tokio_runtime.block_on(async move {
        let cosmos_chain_driver = cosmos_bootstrap.bootstrap_chain("cosmos").await?;

        let celestia_chain_driver = celestia_bootstrap.bootstrap_chain("private").await?;

        let bridge_driver = celestia_bootstrap.bootstrap_bridge(&celestia_chain_driver).await?;

        let _rollup_driver = sovereign_bootstrap
            .bootstrap_rollup(&celestia_chain_driver, &bridge_driver, "test-rollup")
            .await?;

        let cosmos_chain = cosmos_chain_driver.chain();

        let create_client_settings = ClientSettings::Tendermint(Settings {
            max_clock_drift: Duration::from_secs(40),
            trusting_period: None,
            trust_threshold: TrustThreshold::ONE_THIRD,
        });

        let _create_client_payload = <CosmosChain as CanBuildCreateClientPayload<SovereignChain>>::build_create_client_payload(
            &cosmos_chain,
            &create_client_settings,
        ).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
