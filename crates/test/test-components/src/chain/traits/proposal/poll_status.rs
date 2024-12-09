use cgp::prelude::*;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::proposal::types::proposal_status::HasProposalStatusType;

#[cgp_component {
  provider: ProposalStatusPoller,
  context: Chain,
}]
#[async_trait]
pub trait CanPollProposalStatus: HasProposalIdType + HasProposalStatusType + HasErrorType {
    async fn poll_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
        allowed_status: &[Self::ProposalStatus],
    ) -> Result<Self::ProposalStatus, Self::Error>;
}
