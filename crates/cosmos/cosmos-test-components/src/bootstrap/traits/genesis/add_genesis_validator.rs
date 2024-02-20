use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::amount::{AmountOf, HasAmountType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(GenesisValidatorAdderComponent, GenesisValidatorAdder<Bootstrap>)]
#[async_trait]
pub trait CanAddGenesisValidator: HasRuntime + HasChainType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType + HasAmountType,
{
    async fn add_genesis_validator(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
        wallet_id: &str,
        stake_amount: &AmountOf<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
