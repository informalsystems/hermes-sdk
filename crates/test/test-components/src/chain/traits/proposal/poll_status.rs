use cgp_core::prelude::*;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::proposal::types::proposal_status::HasProposalStatusType;

#[derive_component(ProposalStatusPollerComponent, ProposalStatusPoller<Chain>)]
#[async_trait]
pub trait CanPollProposalStatus: HasProposalIdType + HasProposalStatusType + HasErrorType {
    async fn poll_proposal_status<M>(
        &self,
        proposal_id: &Self::ProposalId,
        status_matcher: &M,
    ) -> Result<Self::ProposalStatus, Self::Error>
    where
        M: Fn(&Self::ProposalStatus) -> bool + Async;
}
