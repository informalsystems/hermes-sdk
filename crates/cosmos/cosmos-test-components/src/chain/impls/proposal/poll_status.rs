use core::fmt::{Debug, Display};
use core::time::Duration;

use cgp_core::error::CanRaiseError;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain::traits::proposal::poll_status::ProposalStatusPoller;
use hermes_test_components::chain::traits::proposal::query_status::CanQueryProposalStatus;

pub struct PollProposalStatus;

impl<Chain> ProposalStatusPoller<Chain> for PollProposalStatus
where
    Chain: CanQueryProposalStatus + HasRuntime + CanRaiseError<String>,
    Chain::Runtime: CanSleep,
    Chain::ProposalId: Display,
    Chain::ProposalStatus: Eq + Debug,
{
    async fn poll_proposal_status(
        chain: &Chain,
        proposal_id: &Chain::ProposalId,
        expected_status: &Chain::ProposalStatus,
    ) -> Result<(), Chain::Error> {
        let runtime = chain.runtime();

        for _ in 0..40 {
            let status_result = chain.query_proposal_status(proposal_id).await;

            match &status_result {
                Ok(status) if status == expected_status => {
                    return Ok(());
                }
                _ => {
                    runtime.sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Err(Chain::raise_error(format!(
            "Governance proposal {} was not in status {:?}",
            proposal_id, expected_status
        )))
    }
}
