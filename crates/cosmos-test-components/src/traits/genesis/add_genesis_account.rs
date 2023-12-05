use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::chain::traits::types::address::HasAddressType;
use ibc_test_components::chain::traits::types::amount::HasAmountType;

use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(GenesisAccountAdderComponent, GenesisAccountAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisAccount: HasRuntime + HasAmountType + HasAddressType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn add_genesis_account(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        address: &Self::Address,
        amounts: &[Self::Amount],
    ) -> Result<(), Self::Error>;
}
