use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

/**
   Initialize a new chain with data files stored at the given home directory
*/
#[derive_component(BridgeKeyImporterComponent, BridgeKeyImporter<Bootstrap>)]
#[async_trait]
pub trait CanImportBridgeKey: HasRuntime + HasChainDriverType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn import_bridge_key(
        &self,
        bridge_home_dir: &FilePath<Self::Runtime>,
        chain_driver: &Self::ChainDriver,
    ) -> Result<(), Self::Error>;
}
