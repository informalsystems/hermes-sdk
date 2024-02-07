use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

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
