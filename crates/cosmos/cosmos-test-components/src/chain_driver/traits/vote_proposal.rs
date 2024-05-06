use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[derive_component(GovernanceProposalVoterComponent, GovernanceProposalVoter<ChainDriver>)]
#[async_trait]
pub trait CanVoteProposal:
    HasChainType + HasProposalIdType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasWalletType,
{
    async fn vote_proposal(
        &self,
        proposal_id: &Self::ProposalId,
        sender: &WalletOf<Self::Chain>,
    ) -> Result<String, Self::Error>;
}
