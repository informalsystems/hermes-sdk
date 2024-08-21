use cgp_core::prelude::*;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::proposal::types::proposal_status::HasProposalStatusType;

#[derive_component(ProposalStatusPollerComponent, ProposalStatusPoller<Chain>)]
#[async_trait]
pub trait CanPollProposalStatus: HasProposalIdType + HasProposalStatusType + HasErrorType {
    async fn poll_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
        expected_status: &Self::ProposalStatus,
    ) -> Result<(), Self::Error>;
}
