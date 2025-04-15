use cgp::prelude::*;
use hermes_chain_type_components::traits::{AmountOf, HasAmountType};
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_test_components::chain_driver::traits::HasChainType;

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
