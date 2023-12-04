use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::address::HasAddressType;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::types::io::file_path::HasFilePathType;

#[derive_component(GenesisAccountAdderComponent, GenesisAccountAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisAccount:
    HasFilePathType + HasAmountType + HasAddressType + HasErrorType
{
    async fn add_genesis_account(
        &self,
        chain_home_dir: &Self::FilePath,
        address: &Self::Address,
        amounts: &[Self::Amount],
    ) -> Result<(), Self::Error>;
}
