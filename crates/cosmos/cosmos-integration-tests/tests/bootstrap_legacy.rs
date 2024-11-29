use std::sync::Arc;

use hermes_cosmos_integration_tests::contexts::bootstrap_legacy::LegacyCosmosBootstrap;
use hermes_cosmos_integration_tests::init::init_test_runtime;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_error::types::Error;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;

#[test]
fn test_cosmos_legacy_bootstrap() -> Result<(), Error> {
    let runtime = init_test_runtime();

    // Note: This test only works with Gaia v14 or older. Hence we get the older version of
    // gaiad from the environment variable, if applicable.
    let gaia_bin = std::env::var("LEGACY_GAIA_BIN").unwrap_or("gaiad".into());

    let builder = CosmosBuilder::new_with_default(runtime.clone());

    // TODO: load parameters from environment variables
    let bootstrap = Arc::new(LegacyCosmosBootstrap {
        runtime: runtime.clone(),
        cosmos_builder: builder,
        should_randomize_identifiers: true,
        chain_store_dir: "./test-data".into(),
        chain_command_path: gaia_bin.into(),
        account_prefix: "cosmos".into(),
        compat_mode: None,
        staking_denom_prefix: "stake".into(),
        transfer_denom_prefix: "coin".into(),
        genesis_config_modifier: Box::new(|_| Ok(())),
        comet_config_modifier: Box::new(|_| Ok(())),
    });

    runtime.runtime.clone().block_on(async move {
        let _chain_driver = bootstrap.bootstrap_chain("chain-1").await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
