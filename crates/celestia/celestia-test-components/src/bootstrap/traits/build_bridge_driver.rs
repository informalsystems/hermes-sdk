use cgp::prelude::*;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;

use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;
use crate::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use crate::bridge_driver::traits::bridge_auth_token::{BridgeAuthTokenOf, HasBridgeAuthTokenType};

#[cgp_component {
  name: BridgeDriverBuilderComponent,
  provider: BridgeDriverBuilder,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanBuildBridgeDriver:
    HasBridgeDriverType + HasBridgeConfigType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasChildProcessType,
    Self::BridgeDriver: HasBridgeAuthTokenType,
{
    async fn build_bridge_driver(
        &self,
        bridge_config: Self::BridgeConfig,
        bridge_auth_token: BridgeAuthTokenOf<Self::BridgeDriver>,
        bridge_process: ChildProcessOf<Self::Runtime>,
    ) -> Result<Self::BridgeDriver, Self::Error>;
}
