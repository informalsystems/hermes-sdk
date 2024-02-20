#![recursion_limit = "256"]

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use eyre::{eyre, Error};
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_sovereign_integration_tests::contexts::bootstrap::SovereignBootstrap;
use hermes_sovereign_test_components::bootstrap::traits::bootstrap_rollup::CanBootstrapRollup;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use tokio::runtime::Builder;

#[test]
fn test_sovereign_bootstrap() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let celestia_bootstrap = CelestiaBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        chain_store_dir: format!("./test-data/{store_postfix}/chains").into(),
        bridge_store_dir: format!("./test-data/{store_postfix}/bridges").into(),
    };

    let sovereign_bootstrap = SovereignBootstrap {
        runtime: runtime.clone(),
        rollup_store_dir: format!("./test-data/{store_postfix}/rollups").into(),
        rollup_command_path: "node".into(),
        account_prefix: "sov".into(),
    };

    tokio_runtime.block_on(async move {
        let chain_driver = celestia_bootstrap.bootstrap_chain("private").await?;

        let bridge_driver = celestia_bootstrap.bootstrap_bridge(&chain_driver).await?;

        let rollup_driver = sovereign_bootstrap
            .bootstrap_rollup(&chain_driver, &bridge_driver, "test-rollup")
            .await?;

        {
            // Temporary test to check that rollup driver is bootstrapped properly

            let wallet = rollup_driver
                .wallets
                .get("user-a")
                .ok_or_else(|| eyre!("expect user-a wallet"))?;

            let transfer_denom = &rollup_driver.genesis_config.transfer_token_address;

            let amount = rollup_driver
                .rollup
                .query_balance(&wallet.address, transfer_denom)
                .await?;

            assert_eq!(amount.quantity, 1_000_000_000_000);
        }
        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
