use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::address::{Address, HasAddressType};
use ibc_test_components::chain::traits::types::amount::{Amount, HasAmountType};

use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(GenesisAccountAdderComponent, GenesisAccountAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisAccount: HasRuntime + HasChainType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasAmountType + HasAddressType,
{
    async fn add_genesis_account(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        address: &Address<Self::Chain>,
        amounts: &[Amount<Self::Chain>],
    ) -> Result<(), Self::Error>;
}
