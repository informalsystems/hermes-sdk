use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::runtime::types::file_path::{FilePath, HasFilePathType};

#[derive_component(GenesisValidatorAdderComponent, GenesisValidatorAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisValidator:
    HasRuntime + HasChainIdType + HasAmountType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn add_genesis_validator(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &Self::ChainId,
        wallet_id: &str,
        stake_amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
