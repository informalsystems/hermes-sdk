#![recursion_limit = "256"]

use hermes_core::test_components::test_case::traits::test_case::TestCase;
use hermes_cosmos_integration_tests::contexts::CosmosBinaryChannelTestDriver;
use hermes_cosmos_integration_tests::init::{init_preset_bootstraps, init_test_runtime};
use hermes_error::types::Error;
use hermes_ibc_test_suite::tests::client_refresh::TestRefreshClient;

#[test]
fn test_refresh_client() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        TestRefreshClient::default().run_test(&setup).await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}

// TODO: The current Gaia used, v18, uses a legacy client recovery method
#[cfg(feature = "ibc-go-v8")]
#[test]
fn test_client_recovery() -> Result<(), Error> {
    let runtime = init_test_runtime();

    runtime.runtime.clone().block_on(async move {
        let setup: CosmosBinaryChannelTestDriver =
            init_preset_bootstraps(&runtime, Default::default()).await?;

        hermes_ibc_test_suite::tests::recover_client::TestRecoverClient::default()
            .run_test(&setup)
            .await?;

        <Result<(), Error>>::Ok(())
    })?;

    Ok(())
}
