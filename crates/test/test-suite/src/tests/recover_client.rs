use alloc::format;
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::field::Index;
use hermes_chain_components::traits::{
    CanBuildCreateClientMessage, CanBuildCreateClientPayload, CanBuildUpdateClientMessage,
    CanBuildUpdateClientPayload, CanExtractFromMessageResponse,
    CanOverrideCreateClientPayloadOptions, CanQueryChainHeight,
    CanQueryClientStateWithLatestHeight, CanQueryClientStatus, CanRecoverClient, CanSendMessages,
    CanSendSingleMessage, HasClientStateFields, HasClientStatusMethods, HasCreateClientEvent,
};
use hermes_prelude::*;
use hermes_relayer_components::transaction::traits::CanSendMessagesWithSigner;
use hermes_test_components::chain::traits::{
    CanBuildDepositProposalMessage, CanBuildVoteProposalMessage, CanQueryProposalStatus,
    HasWalletSigner,
};
use hermes_test_components::chain::types::{ProposalStatus, ProposalVote};
use hermes_test_components::chain_driver::traits::{
    CanGenerateRandomAmount, HasWallet, ValidatorWallet,
};
use hermes_test_components::test_case::traits::test_case::TestCase;

use crate::traits::CanUseBinaryTestDriverMethods;

const MAX_RETRIES_FOR_PROPOSAL_STATUS: usize = 15;
const WAIT_SECONDS_FOR_PROPOSAL_STATUS: u64 = 1;

pub struct TestRecoverClient<A = Index<0>, B = Index<1>>(pub PhantomData<(A, B)>);

impl<A, B> Default for TestRecoverClient<A, B> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Driver, A, B> TestCase<Driver> for TestRecoverClient<A, B>
where
    Driver: CanUseBinaryTestDriverMethods<A, B>,
    A: Async,
    B: Async,
{
    async fn run_test(&self, driver: &Driver) -> Result<(), Driver::Error> {
        let chain_a = driver.chain_a();

        let chain_b = driver.chain_b();

        let chain_driver_a = driver.chain_driver_a();

        let validator_wallet = chain_driver_a.wallet(PhantomData::<ValidatorWallet>);

        let create_client_payload_options_b_to_a = driver.create_client_payload_options_b_to_a();

        let subject_create_client_payload_options_b_to_a =
            Driver::ChainB::override_create_client_payload_options(
                create_client_payload_options_b_to_a,
                Duration::from_secs(40),
            );

        let subject_client_payload = chain_b
            .build_create_client_payload(&subject_create_client_payload_options_b_to_a)
            .await
            .map_err(Driver::raise_error)?;

        let create_client_message_options_b_to_a = driver.create_client_message_options_a_to_b();

        let message = chain_a
            .build_create_client_message(
                create_client_message_options_b_to_a,
                subject_client_payload,
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

        let subject_client_id = Driver::ChainA::create_client_event_client_id(&create_client_event);

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

        let denom_a = driver.staking_denom_a();

        let deposit_amount = chain_driver_a.fixed_amount(11000000, denom_a).await;

        // Wait before querying proposal
        tokio::time::sleep(Duration::from_secs(2)).await;

        let proposal_status = chain_a
            .query_proposal_status(&1)
            .await
            .map_err(Driver::raise_error)?;

        if proposal_status == ProposalStatus::DepositPeriod {
            let deposit_message = chain_a.build_deposit_proposal_message(&1, &deposit_amount);

            chain_a
                .send_messages_with_signer(
                    Driver::ChainA::wallet_signer(validator_wallet),
                    &[deposit_message],
                )
                .await
                .map_err(Driver::raise_error)?;
        }

        let mut try_number = 0;
        loop {
            let proposal_status = chain_a
                .query_proposal_status(&1)
                .await
                .map_err(Driver::raise_error)?;

            if proposal_status == ProposalStatus::VotingPeriod {
                let voting_message = chain_a.build_vote_proposal_message(&1, &ProposalVote::Yes);

                chain_a
                    .send_messages_with_signer(
                        Driver::ChainA::wallet_signer(validator_wallet),
                        &[voting_message],
                    )
                    .await
                    .map_err(Driver::raise_error)?;

                break;
            }

            if try_number > MAX_RETRIES_FOR_PROPOSAL_STATUS {
                return Err(Driver::raise_error(format!("Client recovery proposal failed, expected proposal to be in voting period but is {proposal_status:?}")));
            }
            try_number += 1;

            tokio::time::sleep(Duration::from_secs(WAIT_SECONDS_FOR_PROPOSAL_STATUS)).await;
        }

        try_number = 0;
        loop {
            let proposal_status = chain_a
                .query_proposal_status(&1)
                .await
                .map_err(Driver::raise_error)?;

            if proposal_status == ProposalStatus::Passed {
                // Wait before querying client status
                tokio::time::sleep(Duration::from_secs(2)).await;

                let subject_client_status = chain_a
                    .query_client_status(PhantomData, subject_client_id)
                    .await
                    .map_err(Driver::raise_error)?;

                if Driver::ChainB::client_status_is_active(&subject_client_status) {
                    driver
                        .log_message(&format!(
                            "successfully performed client recovery for subject client {subject_client_id} to substitute client {substitute_client_id}"
                        ))
                        .await;

                    return Ok(());
                }
            }
            if try_number == 15 {
                return Err(Driver::raise_error(format!("Client recovery proposal failed, expected proposal to have passed but is {proposal_status:?}")));
            }
            try_number += 1;

            tokio::time::sleep(Duration::from_secs(WAIT_SECONDS_FOR_PROPOSAL_STATUS)).await;
        }
    }
}
