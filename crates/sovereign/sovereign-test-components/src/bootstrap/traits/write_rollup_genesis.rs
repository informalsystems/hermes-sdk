use cgp_core::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::rollup_genesis_config::HasRollupGenesisConfigType;

#[derive_component(RollupGenesisWriterComponent, RollupGenesisWriter<Bootstrap>)]
#[async_trait]
pub trait CanWriteRollupGenesis:
    HasRollupGenesisConfigType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn write_rollup_genesis(
        &self,
        rollup_home_dir: &FilePathOf<Self::Runtime>,
        genesis_config: &Self::RollupGenesisConfig,
    ) -> Result<(), Self::Error>;
}
