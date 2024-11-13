use std::sync::Arc;

use hermes_cosmos_integration_tests::contexts::bootstrap::CosmosBootstrap;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_test_components::types::dynamic_gas_config::DynamicGasConfig;
use hermes_error::types::Error;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;

#[test]
fn test_cosmos_bootstrap() -> Result<(), Error> {
    let maybe_dynamic_gas_fee_config = std::env::var("DYNAMIC_GAS_MULTIPLIER")
        .ok()
        .and_then(|dynamic_gas_multiplier| dynamic_gas_multiplier.parse::<f64>().ok())
        .map(|f64_dynamic_gas_multiplier| DynamicGasConfig {
            multiplier: f64_dynamic_gas_multiplier,
            max: 2.0,
        });

    let runtime = init_test_runtime();

    let builder = Arc::new(CosmosBuilder::new_with_default(runtime.clone()));

    // TODO: load parameters from environment variables
    let bootstrap = Arc::new(CosmosBootstrap {
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
        dynamic_gas: maybe_dynamic_gas_fee_config,
    });

    runtime.runtime.clone().block_on(async move {
        let _chain_driver = bootstrap.bootstrap_chain("chain-1").await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
