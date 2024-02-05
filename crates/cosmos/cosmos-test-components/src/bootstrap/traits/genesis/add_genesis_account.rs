use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain_driver::traits::types::amount::{AmountOf, HasAmountType};

use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(GenesisAccountAdderComponent, GenesisAccountAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisAccount: HasRuntime + HasChainDriverType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::ChainDriver: HasAmountType + HasAddressType,
{
    async fn add_genesis_account(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        address: &AddressOf<Self::ChainDriver>,
        amounts: &[AmountOf<Self::ChainDriver>],
    ) -> Result<(), Self::Error>;
}
