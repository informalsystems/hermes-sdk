use cgp::prelude::*;
use hermes_runtime_components::traits::{
    ChildProcessOf, FilePathOf, HasChildProcessType, HasFilePathType, HasRuntimeType,
};
use hermes_test_components::driver::traits::HasChainDriverType;

use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;

#[cgp_component {
  provider: BridgeStarter,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanStartBridge:
    HasChainDriverType + HasBridgeConfigType + HasRuntimeType + HasAsyncErrorType
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
