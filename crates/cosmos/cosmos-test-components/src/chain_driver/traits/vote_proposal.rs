use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

#[derive_component(GovernanceProposalVoterComponent, GovernanceProposalVoter<ChainDriver>)]
#[async_trait]
pub trait CanVoteProposal: HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn vote_proposal(&self, proposal_id: &str, sender: &str) -> Result<String, Self::Error>;
}
