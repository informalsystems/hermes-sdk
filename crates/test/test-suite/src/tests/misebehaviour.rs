use core::marker::PhantomData;
use core::time::Duration;

use hermes_chain_components::traits::{
    CanBuildUpdateClientMessage, CanBuildUpdateClientPayload, CanQueryChainHeight,
    CanQueryClientStateWithLatestHeight, CanSendMessages, HasClientStateFields,
};
use hermes_prelude::*;
use hermes_test_components::relay_driver::run::CanRunRelayerInBackground;
use hermes_test_components::setup::traits::CanForkFullNode;
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
    Driver: CanUseBinaryTestDriverMethods<A, B> + CanForkFullNode,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let relay_driver = driver.relay_driver();

        let chain_a = driver.chain_a();

        let client_id_a = driver.client_id_a();

        driver
            .log_message("Waiting 10 seconds for chains to produce a few blocks")
            .await;

        tokio::time::sleep(Duration::from_secs(10)).await;

        let forked_setup = driver.fork_full_node().await?;

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

        driver
            .log_message(&alloc::format!(
                "Will build manual client update payload from fork at height: {latest_height_b:?}"
            ))
            .await;

        let update_client_a_payload = chain_b_fork
            .build_update_client_payload(&client_a_state_height, &latest_height_b, client_a_state)
            .await
            .map_err(Driver::raise_error)?;

        let messages = chain_a
            .build_update_client_message(client_id_a, update_client_a_payload)
            .await
            .map_err(Driver::raise_error)?;

        driver
            .log_message(&alloc::format!(
                "Will send manual client update built from fork: {messages:?}"
            ))
            .await;

        let result = chain_a
            .send_messages(messages)
            .await
            .map_err(Driver::raise_error)?;

        driver
            .log_message(&alloc::format!("Manual client update result: {result:?}"))
            .await;

        tokio::time::sleep(core::time::Duration::from_secs(5)).await;

        /*let latest_height_b = chain_b_fork
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let client_a_state = chain_a
            .query_client_state_with_latest_height(PhantomData, client_id_a)
            .await
            .map_err(Driver::raise_error)?;

        let client_a_state_height = Driver::ChainB::client_state_latest_height(&client_a_state);

        driver.log_message(&alloc::format!("Will build second manual client update payload from fork at height: {latest_height_b:?}")).await;

        let update_client_a_payload = chain_b_fork
            .build_update_client_payload(&client_a_state_height, &latest_height_b, client_a_state)
            .await
            .map_err(Driver::raise_error)?;

        let messages = chain_a
            .build_update_client_message(client_id_a, update_client_a_payload)
            .await
            .map_err(Driver::raise_error)?;

        driver.log_message(&alloc::format!("Will send second manual client update built from fork: {messages:?}")).await;

        let result = chain_a
            .send_messages(messages)
            .await
            .map_err(Driver::raise_error)?;

        driver.log_message(&alloc::format!("Second manual client update result: {result:?}")).await;*/

        tokio::time::sleep(core::time::Duration::from_secs(120)).await;

        Ok(())
    }
}
