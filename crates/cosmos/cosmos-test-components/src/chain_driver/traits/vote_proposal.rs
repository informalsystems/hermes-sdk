use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;

#[derive_component(GovernanceProposalVoterComponent, GovernanceProposalVoter<ChainDriver>)]
#[async_trait]
pub trait CanVoteProposal: HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn vote_proposal(&self, proposal_id: &str, sender: &str) -> Result<String, Self::Error>;
}
