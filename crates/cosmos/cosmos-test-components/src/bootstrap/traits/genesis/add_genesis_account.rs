use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain::traits::types::amount::{AmountOf, HasAmountType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

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
