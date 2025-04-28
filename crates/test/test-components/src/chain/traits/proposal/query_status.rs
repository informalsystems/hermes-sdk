use hermes_prelude::*;

use crate::chain::traits::{HasProposalIdType, HasProposalStatusType};

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
