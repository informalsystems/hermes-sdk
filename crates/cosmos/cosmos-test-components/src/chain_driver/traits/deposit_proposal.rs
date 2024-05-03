use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[derive_component(GovernanceProposalDepositerComponent, GovernanceProposalDepositer<ChainDriver>)]
#[async_trait]
pub trait CanDepositProposal: HasChainType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasWalletType,
{
    async fn deposit_proposal(
        &self,
        proposal_id: &str,
        amount: &str,
        sender: &WalletOf<Self::Chain>,
    ) -> Result<String, Self::Error>;
}
