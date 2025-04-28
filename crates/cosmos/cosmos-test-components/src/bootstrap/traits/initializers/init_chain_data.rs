use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::relayer_components::chain::types::aliases::ChainIdOf;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[cgp_component {
  provider: ChainDataInitializer,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanInitChainData: HasRuntime + HasChainType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn init_chain_data(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
