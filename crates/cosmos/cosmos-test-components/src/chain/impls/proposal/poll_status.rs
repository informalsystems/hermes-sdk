use core::fmt::{Debug, Display};
use core::time::Duration;

use cgp::core::error::CanRaiseError;
use cgp::core::Async;
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
    async fn poll_proposal_status<M>(
        chain: &Chain,
        proposal_id: &Chain::ProposalId,
        status_matcher: &M,
    ) -> Result<Chain::ProposalStatus, Chain::Error>
    where
        M: Fn(&Chain::ProposalStatus) -> bool + Async,
    {
        let runtime = chain.runtime();

        for _ in 0..40 {
            let status_result = chain.query_proposal_status(proposal_id).await;

            match status_result {
                Ok(status) if status_matcher(&status) => {
                    return Ok(status);
                }
                _ => {
                    runtime.sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Err(Chain::raise_error(format!(
            "Governance proposal {} was not in expected status",
            proposal_id
        )))
    }
}
