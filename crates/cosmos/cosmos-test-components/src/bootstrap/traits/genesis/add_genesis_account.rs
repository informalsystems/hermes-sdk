use cgp_core::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain::traits::types::amount::{AmountOf, HasAmountType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(GenesisAccountAdderComponent, GenesisAccountAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisAccount: HasRuntime + HasChainType + HasErrorType
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
