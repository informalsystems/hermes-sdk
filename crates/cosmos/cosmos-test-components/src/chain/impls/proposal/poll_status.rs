use core::fmt::{Debug, Display};
use core::time::Duration;

use cgp::core::error::CanRaiseError;
use hermes_relayer_components::error::traits::retry::HasRetryableError;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_test_components::chain::traits::proposal::poll_status::ProposalStatusPoller;
use hermes_test_components::chain::traits::proposal::query_status::CanQueryProposalStatus;

pub struct PollProposalStatus;

impl<Chain> ProposalStatusPoller<Chain> for PollProposalStatus
where
    Chain: CanQueryProposalStatus + HasRuntime + HasRetryableError + CanRaiseError<String>,
    Chain::Runtime: CanSleep,
    Chain::ProposalId: Display,
    Chain::ProposalStatus: Eq + Debug,
{
    async fn poll_proposal_status(
        chain: &Chain,
        proposal_id: &Chain::ProposalId,
        allowed_statuses: &[Chain::ProposalStatus],
    ) -> Result<Chain::ProposalStatus, Chain::Error> {
        let runtime = chain.runtime();

        for _ in 0..40 {
            let status_result = chain.query_proposal_status(proposal_id).await;

            match status_result {
                Ok(status) => {
                    for allowed_status in allowed_statuses {
                        if allowed_status == &status {
                            return Ok(status);
                        } else {
                            runtime.sleep(Duration::from_millis(500)).await;
                        }
                    }
                }
                Err(e) => {
                    if Chain::is_retryable_error(&e) {
                        runtime.sleep(Duration::from_millis(500)).await;
                    } else {
                        return Err(e);
                    }
                }
            }
        }

        Err(Chain::raise_error(format!(
            "Governance proposal {} was not in expected status {:?}",
            proposal_id, allowed_statuses,
        )))
    }
}
