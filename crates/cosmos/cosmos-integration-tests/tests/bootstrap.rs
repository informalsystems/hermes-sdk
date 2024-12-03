use std::sync::Arc;

use hermes_cosmos_chain_components::types::config::gas::dynamic_gas_config::DynamicGasConfig;
use hermes_cosmos_integration_tests::contexts::bootstrap::{
    CosmosBootstrap, CosmosBootstrapFields,
};
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;

#[test]
fn test_cosmos_bootstrap() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let builder = CosmosBuilder::new_with_default(runtime.clone());

    // TODO: load parameters from environment variables
    let bootstrap = CosmosBootstrap {
        fields: Arc::new(CosmosBootstrapFields {
            runtime: runtime.clone(),
            cosmos_builder: builder,
            should_randomize_identifiers: true,
            chain_store_dir: "./test-data".into(),
            chain_command_path: "gaiad".into(),
            account_prefix: "cosmos".into(),
            staking_denom_prefix: "stake".into(),
            transfer_denom_prefix: "coin".into(),
            genesis_config_modifier: Box::new(|_| Ok(())),
            comet_config_modifier: Box::new(|_| Ok(())),
            dynamic_gas: Some(DynamicGasConfig::default()),
        }),
    };

    runtime.runtime.clone().block_on(async move {
        let _chain_driver = bootstrap.bootstrap_chain("chain-1").await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
