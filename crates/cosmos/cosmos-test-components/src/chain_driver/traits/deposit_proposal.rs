use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;

#[derive_component(GovernanceProposalDepositerComponent, GovernanceProposalDepositer<ChainDriver>)]
#[async_trait]
pub trait CanDepositProposal: HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn deposit_proposal(
        &self,
        proposal_id: &str,
        amount: &str,
        sender: &str,
    ) -> Result<String, Self::Error>;
}
