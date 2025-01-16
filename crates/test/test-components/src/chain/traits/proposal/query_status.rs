use cgp::prelude::*;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::proposal::types::proposal_status::HasProposalStatusType;

#[cgp_component {
  provider: ProposalStatusQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryProposalStatus:
    HasProposalIdType + HasProposalStatusType + HasAsyncErrorType
{
    async fn query_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
    ) -> Result<Self::ProposalStatus, Self::Error>;
}
