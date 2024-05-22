use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;

use crate::chain::traits::proposal::types::proposal_id::{HasProposalIdType, ProposalIdOf};
use crate::chain::traits::types::wallet::{HasWalletType, WalletOf};
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive_component(ProposalVoterComponent, ProposalVoter<ChainDriver>)]
#[async_trait]
pub trait CanVoteProposal: HasChainType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasWalletType + HasProposalIdType,
{
    async fn vote_proposal(
        &self,
        proposal_id: &ProposalIdOf<Self::Chain>,
        sender: &WalletOf<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
