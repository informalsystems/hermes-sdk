use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::relayer_components::chain::types::aliases::ChainIdOf;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

#[cgp_component {
  provider: ChainHomeDirInitializer,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanInitChainHomeDir: HasChainType + HasRuntime + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn init_chain_home_dir(
        &self,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<FilePathOf<Self::Runtime>, Self::Error>;
}
