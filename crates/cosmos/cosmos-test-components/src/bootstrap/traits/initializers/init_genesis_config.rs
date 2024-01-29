use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::genesis_config::HasGenesisConfigType;

#[derive_component(GenesisConfigInitializerComponent, GenesisConfigInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitGenesisConfig: HasRuntime + HasGenesisConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_genesis_config(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<Self::GenesisConfig, Self::Error>;
}
