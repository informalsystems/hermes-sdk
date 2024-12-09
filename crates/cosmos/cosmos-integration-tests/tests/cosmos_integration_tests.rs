#![recursion_limit = "256"]

use hermes_cosmos_integration_tests::contexts::binary_channel::test_driver::CosmosBinaryChannelTestDriver;
use hermes_cosmos_integration_tests::init::{init_preset_bootstraps, init_test_runtime};
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::transfer::TestIbcTransfer;
use hermes_test_components::test_case::traits::test_case::TestCase;

#[test]
fn cosmos_integration_tests() -> Result<(), Error> {
    let runtime = init_test_runtime();

    // TODO: Use a test suite entry point for running multiple tests
    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver = init_preset_bootstraps(&runtime).await?;
        TestIbcTransfer::run_test(&TestIbcTransfer, &setup).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
