use cgp::prelude::*;
use hermes_core::chain_type_components::traits::{
    AddressOf, AmountOf, HasAddressType, HasAmountType,
};
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;

#[cgp_component {
  provider: GenesisAccountAdder,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanAddGenesisAccount: HasRuntime + HasChainType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasAmountType + HasAddressType,
{
    async fn add_genesis_account(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        address: &AddressOf<Self::Chain>,
        amounts: &[AmountOf<Self::Chain>],
    ) -> Result<(), Self::Error>;
}
