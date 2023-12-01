use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::types::file_path::HasFilePathType;

#[derive_component(GenesisValidatorAdderComponent, GenesisValidatorAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisValidator:
    HasChainIdType + HasFilePathType + HasAmountType + HasErrorType
{
    async fn add_genesis_validator(
        &self,
        chain_home_dir: &Self::FilePath,
        chain_id: &Self::ChainId,
        wallet_id: &str,
        stake_amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
