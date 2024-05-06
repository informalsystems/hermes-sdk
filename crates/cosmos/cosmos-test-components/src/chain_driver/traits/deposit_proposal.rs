use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::types::amount::{AmountOf, HasAmountType};
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[derive_component(GovernanceProposalDepositerComponent, GovernanceProposalDepositer<ChainDriver>)]
#[async_trait]
pub trait CanDepositProposal:
    HasChainType + HasProposalIdType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasWalletType + HasAmountType,
{
    async fn deposit_proposal(
        &self,
        proposal_id: &Self::ProposalId,
        amount: &AmountOf<Self::Chain>,
        sender: &WalletOf<Self::Chain>,
    ) -> Result<String, Self::Error>;
}
