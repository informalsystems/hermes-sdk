use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

#[derive_component(ChainGenesisConfigInitializerComponent, ChainGenesisConfigInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitChainGenesisConfig: HasRuntime + HasChainGenesisConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_genesis_config(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<Self::ChainGenesisConfig, Self::Error>;
}
