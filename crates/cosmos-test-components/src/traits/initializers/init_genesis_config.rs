use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::traits::types::genesis_config::HasGenesisConfigType;
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(GenesisConfigInitializerComponent, GenesisConfigInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitGenesisConfig: HasRuntime + HasGenesisConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_genesis_config(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
    ) -> Result<Self::GenesisConfig, Self::Error>;
}
