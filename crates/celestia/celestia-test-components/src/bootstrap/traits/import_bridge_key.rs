use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[cgp_component {
  provider: BridgeKeyImporter,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanImportBridgeKey: HasRuntime + HasChainDriverType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn import_bridge_key(
        &self,
        bridge_home_dir: &FilePathOf<Self::Runtime>,
        chain_driver: &Self::ChainDriver,
    ) -> Result<(), Self::Error>;
}
