use crate::chain::traits::proposal::types::proposal_id::{HasProposalIdType, ProposalIdOf};
use crate::chain::traits::proposal::types::proposal_status::{
    HasProposalStatusType, ProposalStatusOf,
};
use crate::chain_driver::traits::types::chain::HasChainType;
use cgp_core::prelude::*;

#[derive_component(ProposalStatusPollerComponent, ProposalStatusPoller<ChainDriver>)]
#[async_trait]
pub trait CanPollProposalStatus: HasChainType + HasErrorType
where
    Self::Chain: HasProposalIdType + HasProposalStatusType,
{
    async fn poll_proposal_status(
        &self,
        proposal_id: &ProposalIdOf<Self::Chain>,
        expected_status: &ProposalStatusOf<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
