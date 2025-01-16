use cgp::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;

#[cgp_component {
  provider: BridgeBootstrapper,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanBootstrapBridge:
    HasChainDriverType + HasBridgeDriverType + HasRuntime + HasAsyncErrorType
{
    async fn bootstrap_bridge(
        &self,
        chain_driver: &Self::ChainDriver,
    ) -> Result<Self::BridgeDriver, Self::Error>;
}
