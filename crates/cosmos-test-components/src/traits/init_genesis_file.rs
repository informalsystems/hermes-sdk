use cgp_core::prelude::*;

use crate::traits::file_path::HasFilePathType;

#[derive_component(GenesisFileInitializerComponent, GenesisFileInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitGenesisFile: HasFilePathType + HasErrorType {
    async fn init_genesis_file(&self, chain_home_dir: &Self::FilePath) -> Result<(), Self::Error>;
}
