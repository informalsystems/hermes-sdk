use core::fmt::{Debug, Display};
use core::time::Duration;

use hermes_core::relayer_components::error::traits::HasRetryableError;
use hermes_core::runtime_components::traits::{CanSleep, HasRuntime};
use hermes_core::test_components::chain::traits::{
    CanQueryProposalStatus, ProposalStatusPoller, ProposalStatusPollerComponent,
};
use hermes_prelude::*;

pub struct PollProposalStatus;

#[cgp_provider(ProposalStatusPollerComponent)]
impl<Chain> ProposalStatusPoller<Chain> for PollProposalStatus
where
    Chain: CanQueryProposalStatus + HasRuntime + HasRetryableError + CanRaiseAsyncError<String>,
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
            "Governance proposal {proposal_id} was not in expected status {allowed_statuses:?}"
        )))
    }
}
