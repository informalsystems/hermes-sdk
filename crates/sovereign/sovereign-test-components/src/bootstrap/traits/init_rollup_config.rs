use cgp_core::prelude::*;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::rollup_config::HasRollupConfigType;

#[derive_component(RollupConfigInitializerComponent, RollupConfigInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitRollupConfig:
    HasRuntimeType + HasBridgeDriverType + HasRollupConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_rollup_config(
        &self,
        rollup_home_dir: &FilePathOf<Self::Runtime>,
        bridge_driver: &Self::BridgeDriver,
    ) -> Result<Self::RollupConfig, Self::Error>;
}
