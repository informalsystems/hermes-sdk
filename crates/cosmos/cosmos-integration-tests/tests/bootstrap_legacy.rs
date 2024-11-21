use std::sync::Arc;

use hermes_cosmos_integration_tests::init::{init_bootstrap_legacy, init_test_runtime};
use hermes_error::types::Error;
use hermes_test_components::bootstrap::traits::chain::CanBootstrapChain;

#[test]
fn test_cosmos_legacy_bootstrap() -> Result<(), Error> {
    let runtime = init_test_runtime();

    let dynamic_gas = None;

    let bootstrap = Arc::new(init_bootstrap_legacy(
        0,
        runtime.clone(),
        true,
        "./test-data",
        "coin".into(),
        |_| Ok(()),
        |_| Ok(()),
        dynamic_gas,
    ));

    runtime.runtime.clone().block_on(async move {
        let _chain_driver = bootstrap.bootstrap_chain("chain-1").await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
