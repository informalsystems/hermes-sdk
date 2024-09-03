use cgp::prelude::*;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::proposal::types::proposal_status::HasProposalStatusType;

#[derive_component(ProposalStatusQuerierComponent, ProposalStatusQuerier<Chain>)]
#[async_trait]
pub trait CanQueryProposalStatus: HasProposalIdType + HasProposalStatusType + HasErrorType {
    async fn query_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
    ) -> Result<Self::ProposalStatus, Self::Error>;
}
