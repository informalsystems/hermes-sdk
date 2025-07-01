use core::marker::PhantomData;
use core::time::Duration;

use hermes_core::chain_components::traits::{
    CanQueryClientStatus, HasAmountType, HasClientIdType, HasClientStatusMethods, HasDenomType,
};
use hermes_core::logging_components::traits::CanLogMessage;
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_core::relayer_components::transaction::traits::CanSendMessagesWithSigner;
use hermes_prelude::*;
use hermes_test_components::chain::traits::{
    CanBuildDepositProposalMessage, CanBuildVoteProposalMessage, CanQueryProposalStatus,
    HasWalletSigner,
};
use hermes_test_components::chain::types::{ProposalStatus, ProposalVote};
use hermes_test_components::chain_driver::traits::{
    CanGenerateRandomAmount, HasChain, HasDenom, HasWallet, StakingDenom, ValidatorWallet,
};
use hermes_test_components::driver::traits::HasChainDriverAt;
use hermes_test_components::test_case::traits::recover_client::{
    RecoverClientHandler, RecoverClientHandlerComponent,
};
use ibc::core::host::types::identifiers::ClientId;

const MAX_RETRIES_FOR_PROPOSAL_STATUS: usize = 15;
const WAIT_SECONDS_FOR_PROPOSAL_STATUS: u64 = 1;

pub struct RecoverClientWithProposals;

#[cgp_provider(RecoverClientHandlerComponent)]
impl<Driver, ChainDriverA, ChainA, ChainB>
    RecoverClientHandler<Driver, ChainDriverA, ChainA, ChainB> for RecoverClientWithProposals
where
    Driver: HasChainTypeAt<Index<1>, Chain = ChainB>
        + HasChainDriverAt<Index<0>, ChainDriver = ChainDriverA>
        + CanLogMessage
        + CanRaiseAsyncError<String>,
    ChainDriverA: HasChain<Chain = ChainA>
        + HasDenom<StakingDenom>
        + HasWallet<ValidatorWallet>
        + CanGenerateRandomAmount,
    ChainA: CanQueryProposalStatus<ProposalId = u64, ProposalStatus = ProposalStatus>
        + CanQueryClientStatus<ChainB>
        + CanBuildDepositProposalMessage
        + CanBuildVoteProposalMessage<ProposalVote = ProposalVote>
        + CanSendMessagesWithSigner
        + HasClientIdType<ChainB, ClientId = ClientId>
        + HasAmountType
        + HasDenomType
        + HasWalletSigner,
    ChainB: HasClientStatusMethods<ChainA>,
{
    async fn handle_recover_client(
        driver: &Driver,
        subject_client_id: &ClientId,
        substitute_client_id: &ClientId,
    ) -> Result<(), Driver::Error> {
        let chain_driver_a = driver.chain_driver_at(PhantomData::<Index<0>>);

        let chain_a = chain_driver_a.chain();

        let validator_wallet = chain_driver_a.wallet(PhantomData::<ValidatorWallet>);

        let denom_a = chain_driver_a.denom(PhantomData::<StakingDenom>);

        let deposit_amount = chain_driver_a.fixed_amount(11000000, denom_a).await;

        let proposal_status = chain_a
            .query_proposal_status(&1)
            .await
            .map_err(|e| Driver::raise_error(format!("{e:?}")))?;

        if proposal_status == ProposalStatus::DepositPeriod {
            let deposit_message = chain_a.build_deposit_proposal_message(&1, &deposit_amount);

            chain_a
                .send_messages_with_signer(
                    ChainA::wallet_signer(validator_wallet),
                    &[deposit_message],
                )
                .await
                .map_err(|e| Driver::raise_error(format!("{e:?}")))?;
        }

        let mut try_number = 0;
        loop {
            let proposal_status = chain_a
                .query_proposal_status(&1)
                .await
                .map_err(|e| Driver::raise_error(format!("{e:?}")))?;

            if proposal_status == ProposalStatus::VotingPeriod {
                let voting_message = chain_a.build_vote_proposal_message(&1, &ProposalVote::Yes);

                chain_a
                    .send_messages_with_signer(
                        ChainA::wallet_signer(validator_wallet),
                        &[voting_message],
                    )
                    .await
                    .map_err(|e| Driver::raise_error(format!("{e:?}")))?;

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
                .map_err(|e| Driver::raise_error(format!("{e:?}")))?;

            if proposal_status == ProposalStatus::Passed {
                // Wait before querying client status
                tokio::time::sleep(Duration::from_secs(2)).await;

                let subject_client_status = chain_a
                    .query_client_status(PhantomData, subject_client_id)
                    .await
                    .map_err(|e| Driver::raise_error(format!("{e:?}")))?;

                if ChainB::client_status_is_active(&subject_client_status) {
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
