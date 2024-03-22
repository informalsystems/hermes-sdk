use cgp_core::prelude::*;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;

use crate::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;

#[derive_component(RollupNodeConfigInitializerComponent, RollupNodeConfigInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitRollupNodeConfig:
    HasRuntimeType + HasBridgeDriverType + HasRollupNodeConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_rollup_node_config(
        &self,
        rollup_home_dir: &FilePathOf<Self::Runtime>,
        bridge_driver: &Self::BridgeDriver,
    ) -> Result<Self::RollupNodeConfig, Self::Error>;
}
