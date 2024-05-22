use cgp_core::prelude::*;

use crate::chain::traits::proposal::types::proposal_id::{HasProposalIdType, ProposalIdOf};
use crate::chain::traits::proposal::types::proposal_status::{
    HasProposalStatusType, ProposalStatusOf,
};
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive_component(ProposalStatusQuerierComponent, ProposalStatusQuerier<ChainDriver>)]
#[async_trait]
pub trait CanQueryProposalStatus: HasChainType + HasErrorType
where
    Self::Chain: HasProposalIdType + HasProposalStatusType,
{
    async fn query_proposal_status(
        &self,
        proposal_id: &ProposalIdOf<Self::Chain>,
    ) -> Result<ProposalStatusOf<Self::Chain>, Self::Error>;
}
