use cgp::prelude::*;
use hermes_runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntimeType};
use hermes_test_components::driver::traits::HasChainDriverType;

use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;

#[cgp_component {
  provider: BridgeConfigInitializer,
  context: Boostrap,
}]
#[async_trait]
pub trait CanInitBridgeConfig:
    HasRuntimeType + HasChainDriverType + HasBridgeConfigType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_bridge_config(
        &self,
        bridge_home_dir: &FilePathOf<Self::Runtime>,
        chain_driver: &Self::ChainDriver,
    ) -> Result<Self::BridgeConfig, Self::Error>;
}
