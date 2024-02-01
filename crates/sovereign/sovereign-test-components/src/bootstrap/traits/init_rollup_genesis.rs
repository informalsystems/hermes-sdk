use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;

#[derive_component(RollupGenesisInitializerComponent, RollupGenesisInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitRollupGenesis: HasRollupGenesisConfigType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_rollup_genesis(
        &self,
        rollup_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<Self::RollupGenesisConfig, Self::Error>;
}
