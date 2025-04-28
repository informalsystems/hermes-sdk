use hermes_core::chain_type_components::traits::{AmountOf, HasAmountType};
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::relayer_components::chain::types::aliases::ChainIdOf;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

#[cgp_component {
  provider: GenesisValidatorAdder,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanAddGenesisValidator: HasRuntime + HasChainType + HasAsyncErrorType
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
