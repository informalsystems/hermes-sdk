use cgp_core::prelude::*;

use crate::traits::types::genesis_config::HasGenesisConfigType;
use crate::traits::types::io::file_path::HasFilePathType;

#[derive_component(GenesisConfigInitializerComponent, GenesisConfigInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitGenesisConfig: HasFilePathType + HasGenesisConfigType + HasErrorType {
    async fn init_genesis_config(
        &self,
        chain_home_dir: &Self::FilePath,
    ) -> Result<Self::GenesisConfig, Self::Error>;
}
