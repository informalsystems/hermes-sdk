use alloc::format;
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use hermes_chain_components::traits::{
    CanBuildCreateClientMessage, CanBuildCreateClientPayload, CanBuildUpdateClientMessage,
    CanBuildUpdateClientPayload, CanExtractFromMessageResponse, CanQueryChainHeight,
    CanQueryClientStateWithLatestHeight, CanQueryClientStatus, CanRecoverClient, CanSendMessages,
    CanSendSingleMessage, HasClientStateFields, HasClientStatusMethods, HasCreateClientEvent,
};
use hermes_prelude::*;
use hermes_test_components::test_case::traits::recover_client::CanHandleRecoverClient;
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

pub struct TestRecoverClient<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestRecoverClient<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestRecoverClient<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>
        + CanHandleRecoverClient<Driver::ChainDriverA, Driver::ChainA, Driver::ChainB>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_a = driver.chain_a();

        let chain_b = driver.chain_b();

        let subject_client_id = driver.client_id_a();

        let latest_height_b = chain_b
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let subject_client_state = chain_a
            .query_client_state_with_latest_height(PhantomData, subject_client_id)
            .await
            .map_err(Driver::raise_error)?;

        let subject_client_state_height =
            Driver::ChainB::client_state_latest_height(&subject_client_state);

        let subject_update_payload = chain_b
            .build_update_client_payload(
                &subject_client_state_height,
                &latest_height_b,
                subject_client_state,
            )
            .await
            .map_err(Driver::raise_error)?;

        let messages = chain_a
            .build_update_client_message(subject_client_id, subject_update_payload)
            .await
            .map_err(Driver::raise_error)?;

        chain_a
            .send_messages(messages)
            .await
            .map_err(Driver::raise_error)?;

        tokio::time::sleep(Duration::from_secs(45)).await;

        let subject_client_status = chain_a
            .query_client_status(PhantomData, subject_client_id)
            .await
            .map_err(Driver::raise_error)?;

        assert!(
            Driver::ChainB::client_status_is_expired(&subject_client_status),
            "expected subject client to be expired before recover process"
        );

        let create_client_message_options_b_to_a = driver.create_client_message_options_a_to_b();

        let substituate_create_client_payload_options_b_to_a =
            driver.create_client_payload_options_b_to_a();

        let substitute_client_payload = chain_b
            .build_create_client_payload(substituate_create_client_payload_options_b_to_a)
            .await
            .map_err(Driver::raise_error)?;

        let message = chain_a
            .build_create_client_message(
                create_client_message_options_b_to_a,
                substitute_client_payload,
            )
            .await
            .map_err(Driver::raise_error)?;

        let response = chain_a
            .send_message(message)
            .await
            .map_err(Driver::raise_error)?;

        let create_client_event = chain_a
            .try_extract_from_message_response(PhantomData, &response)
            .ok_or_else(|| format!("failed to extract client ID from response: {response:?}"))
            .map_err(Driver::raise_error)?;

        let substitute_client_id =
            Driver::ChainA::create_client_event_client_id(&create_client_event);

        let substitute_client_status = chain_a
            .query_client_status(PhantomData, substitute_client_id)
            .await
            .map_err(Driver::raise_error)?;

        assert!(
            Driver::ChainB::client_status_is_active(&substitute_client_status),
            "expected substitute client to be active"
        );

        tokio::time::sleep(Duration::from_secs(10)).await;

        let latest_height_b = chain_b
            .query_chain_height()
            .await
            .map_err(Driver::raise_error)?;

        let substitute_client_state = chain_a
            .query_client_state_with_latest_height(PhantomData, substitute_client_id)
            .await
            .map_err(Driver::raise_error)?;

        let substitute_client_state_height =
            Driver::ChainB::client_state_latest_height(&substitute_client_state);

        let substitute_update_payload = chain_b
            .build_update_client_payload(
                &substitute_client_state_height,
                &latest_height_b,
                substitute_client_state,
            )
            .await
            .map_err(Driver::raise_error)?;

        let messages = chain_a
            .build_update_client_message(substitute_client_id, substitute_update_payload)
            .await
            .map_err(Driver::raise_error)?;

        chain_a
            .send_messages(messages)
            .await
            .map_err(Driver::raise_error)?;

        let recover_client_payload = driver.recover_client_payload_options_a();

        let message = chain_a
            .recover_client_message(
                subject_client_id,
                substitute_client_id,
                recover_client_payload,
            )
            .await;

        chain_a
            .send_message(message)
            .await
            .map_err(Driver::raise_error)?;

        driver
            .handle_recover_client(subject_client_id, substitute_client_id)
            .await
    }
}
