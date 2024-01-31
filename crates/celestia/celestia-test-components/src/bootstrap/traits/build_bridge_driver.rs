use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};

use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;
use crate::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use crate::bridge_driver::traits::bridge_auth_token::{BridgeAuthTokenOf, HasBridgeAuthTokenType};

#[derive_component(BridgeDriverBuilderComponent, BridgeDriverBuilder<Bootstrap>)]
#[async_trait]
pub trait CanBuildBridgeDriver:
    HasBridgeDriverType + HasBridgeConfigType + HasRuntime + HasErrorType
where
    Self::Runtime: HasChildProcessType,
    Self::BridgeDriver: HasBridgeAuthTokenType,
{
    async fn build_bridge_driver(
        &self,
        bridge_config: Self::BridgeConfig,
        bridge_auth_token: BridgeAuthTokenOf<Self::BridgeDriver>,
        bridge_process: ChildProcess<Self::Runtime>,
    ) -> Result<Self::BridgeDriver, Self::Error>;
}
