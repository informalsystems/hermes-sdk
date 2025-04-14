use cgp::prelude::*;
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::HasChainType;

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[cgp_component {
  provider: BridgeDataInitializer,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanInitBridgeData: HasRuntime + HasChainType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn init_bridge_data(
        &self,
        bridge_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
