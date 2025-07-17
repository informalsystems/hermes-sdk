use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use hermes_chain_components::traits::{CanQueryClientStatus, HasClientStatusMethods};
use hermes_prelude::*;
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::test_case::traits::recover_client::CanHandleRecoverClient;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestRefreshClient<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestRefreshClient<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestRefreshClient<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>
        + CanHandleRecoverClient<Driver::ChainDriverA, Driver::ChainA, Driver::ChainB>
        + CanRaiseAsyncError<&'static str>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let relay_driver = driver.relay_driver();

        let chain_a = driver.chain_a();

        let client_id_a = driver.client_id_a();

        // Start relayer
        let _handle = relay_driver
            .run_relayer_in_background()
            .await
            .map_err(Driver::raise_error)?;

        // Loop during 50 seconds to verify that the client is automatically refreshed and
        // doesn't expire
        for _ in 0..10 {
            let client_a_client_status = chain_a
                .query_client_status(PhantomData, client_id_a)
                .await
                .map_err(Driver::raise_error)?;

            assert!(
                Driver::ChainB::client_status_is_active(&client_a_client_status),
                "expected client to be automatically refreshed"
            );

            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        drop(_handle);

        // Sanity check:
        // Loop during 50 seconds to verify that after stopping the auto relayer
        // the client eventually expires
        for _ in 0..10 {
            let client_a_client_status = chain_a
                .query_client_status(PhantomData, client_id_a)
                .await
                .map_err(Driver::raise_error)?;

            if Driver::ChainB::client_status_is_expired(&client_a_client_status) {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_secs(5)).await;
        }

        Err(Driver::raise_error(
            "Sanity check for client refresh failed",
        ))
    }
}
