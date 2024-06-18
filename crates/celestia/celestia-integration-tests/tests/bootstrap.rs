#![recursion_limit = "256"]

use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use eyre::Error;
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_celestia_test_components::bootstrap::traits::bootstrap_bridge::CanBootstrapBridge;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use tokio::runtime::Builder;

#[test]
fn test_celestia_bootstrap() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

    let store_dir = std::env::current_dir()?.join(format!("test-data/{store_postfix}"));

    let celestia_bootstrap = CelestiaBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        chain_store_dir: store_dir.join("chains"),
        bridge_store_dir: store_dir.join("bridges"),
    };

    tokio_runtime.block_on(async move {
        let chain_driver = celestia_bootstrap.bootstrap_chain("private").await?;

        // Bootstrap twice to ensure that there is no conflict such as listening port
        let _bridge_driver = celestia_bootstrap.bootstrap_bridge(&chain_driver).await?;
        let _bridge_driver_b = celestia_bootstrap.bootstrap_bridge(&chain_driver).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
