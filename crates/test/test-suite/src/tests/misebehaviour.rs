use alloc::string::ToString;
use core::marker::PhantomData;
use core::time::Duration;

use hermes_chain_components::traits::{
    CanBuildUpdateClientMessage, CanBuildUpdateClientPayload, CanQueryChainHeight,
    CanQueryClientStateWithLatestHeight, CanQueryClientStatus, CanSendMessages, HasChainId,
    HasClientStateFields, HasClientStatusMethods,
};
use hermes_prelude::*;
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::setup::traits::CanForkFullNode;
use hermes_test_components::test_case::traits::node::CanHaltFullNode;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestMisbehaviourDetection<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestMisbehaviourDetection<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestMisbehaviourDetection<A, B>
where
    Driver:
        CanUseBinaryTestDriverMethods<A, B> + CanForkFullNode + CanRaiseAsyncError<&'static str>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let relay_driver = driver.relay_driver();

        let chain_driver_a = driver.chain_driver_a();
        let chain_driver_b = driver.chain_driver_b();

        let chain_a = driver.chain_a();

        let client_id_a = driver.client_id_a();

        driver
            .log_message("Waiting 10 seconds for chains to produce a few blocks")
            .await;

        tokio::time::sleep(Duration::from_secs(10)).await;

        chain_driver_a
            .halt_full_node()
            .await
            .map_err(Driver::raise_error)?;
        chain_driver_b
            .halt_full_node()
            .await
            .map_err(Driver::raise_error)?;

        let forked_setup = driver
            .fork_full_node(driver.chain_b().chain_id().to_string())
            .await?;

        tokio::time::sleep(Duration::from_secs(10)).await;

        // Start relayer
        let _handle = relay_driver
            .run_relayer_in_background()
            .await
            .map_err(Driver::raise_error)?;

        driver.log_message("Waiting for Hermes to start").await;

        tokio::time::sleep(Duration::from_secs(10)).await;

        let chain_b_fork = forked_setup.chain_b();

        let latest_height_b = chain_b_fork
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let client_a_state = chain_a
            .query_client_state_with_latest_height(PhantomData, client_id_a)
            .await
            .map_err(Driver::raise_error)?;

        let client_a_state_height = Driver::ChainB::client_state_latest_height(&client_a_state);

        let update_client_a_payload = chain_b_fork
            .build_update_client_payload(&client_a_state_height, &latest_height_b, client_a_state)
            .await
            .map_err(Driver::raise_error)?;

        let messages = chain_a
            .build_update_client_message(client_id_a, update_client_a_payload)
            .await
            .map_err(Driver::raise_error)?;

        tokio::time::sleep(core::time::Duration::from_secs(2)).await;

        chain_a
            .send_messages(messages)
            .await
            .map_err(Driver::raise_error)?;

        tokio::time::sleep(core::time::Duration::from_secs(5)).await;

        driver
            .log_message("Will assert the client is eventually frozen")
            .await;

        for _ in 0..100 {
            let client_status = chain_a
                .query_client_status(PhantomData, client_id_a)
                .await
                .map_err(Driver::raise_error)?;

            if Driver::ChainB::client_status_is_frozen(&client_status) {
                driver
                    .log_message(
                        "Client is frozen after misbehaviour has been detected and submitted",
                    )
                    .await;

                return Ok(());
            }
            tokio::time::sleep(core::time::Duration::from_secs(1)).await;
        }

        Err(Driver::raise_error(
            "Failed to detect and submit misbehaviour",
        ))
    }
}
