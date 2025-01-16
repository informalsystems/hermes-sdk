use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

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
