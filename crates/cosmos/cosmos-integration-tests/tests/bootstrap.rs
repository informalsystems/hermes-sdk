use std::sync::Arc;

use hermes_core::test_components::bootstrap::traits::CanBootstrapChain;
use hermes_cosmos_integration_tests::init::{
    build_gaia_bootstrap, build_osmosis_bootstrap, init_test_runtime,
};
use hermes_error::types::Error;

#[test]
fn test_cosmos_bootstrap() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let bootstrap = Arc::new(build_gaia_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        Default::default(),
    ));

    let bootstrap_legacy = Arc::new(build_osmosis_bootstrap(
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        Default::default(),
    ));

    runtime.runtime.clone().block_on(async move {
        let _chain_driver = bootstrap.bootstrap_chain("chain-1").await?;
        let _chain_driver = bootstrap_legacy.bootstrap_chain("chain-2").await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
