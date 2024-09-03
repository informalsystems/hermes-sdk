use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;

#[derive_component(BridgeStarterComponent, BridgeStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartBridge:
    HasChainDriverType + HasBridgeConfigType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasChildProcessType + HasFilePathType,
{
    async fn start_bridge(
        &self,
        bridge_home_dir: &FilePathOf<Self::Runtime>,
        bridge_config: &Self::BridgeConfig,
        chain_driver: &Self::ChainDriver,
    ) -> Result<ChildProcessOf<Self::Runtime>, Self::Error>;
}
