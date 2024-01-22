#![recursion_limit = "256"]

use std::sync::Arc;

use eyre::Error;
use hermes_celestia_integration_tests::contexts::bootstrap::CelestiaBootstrap;
use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_test_components::chain_driver::types::denom::Denom;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;
use ibc_relayer::config::compat_mode::CompatMode;
use rand::prelude::*;
use tokio::runtime::Builder;

#[test]
fn test_celestia_bootstrap() -> Result<(), Error> {
    let _ = stable_eyre::install();

    let tokio_runtime = Arc::new(Builder::new_multi_thread().enable_all().build()?);

    let runtime = HermesRuntime::new(tokio_runtime.clone());

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    let store_postfix: u32 = {
        let mut rng = thread_rng();
        rng.gen()
    };

    let cosmos_bootstrap = CosmosBootstrap {
        runtime: runtime.clone(),
        builder: builder.clone(),
        should_randomize_identifiers: false,
        chain_store_dir: format!("./test-data/{store_postfix}/chains").into(),
        chain_command_path: "celestia-appd".into(),
        account_prefix: "celestia".into(),
        compat_mode: Some(CompatMode::V0_34),
        staking_denom: Denom::base("utia"),
        transfer_denom: Denom::base("coin"),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    };

    let celestia_bootstrap = CelestiaBootstrap {
        cosmos_bootstrap,
        bridge_store_dir: format!("./test-data/{store_postfix}/bridges").into(),
    };

    tokio_runtime.block_on(async move {
        celestia_bootstrap.bootstrap_chain("private").await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
