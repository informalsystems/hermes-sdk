use hermes_prelude::*;

use crate::chain::traits::{HasProposalIdType, HasProposalStatusType};

#[cgp_component {
  provider: ProposalStatusPoller,
  context: Chain,
}]
#[async_trait]
pub trait CanPollProposalStatus:
    HasProposalIdType + HasProposalStatusType + HasAsyncErrorType
{
    async fn poll_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
        allowed_status: &[Self::ProposalStatus],
    ) -> Result<Self::ProposalStatus, Self::Error>;
}
