use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;

#[derive_component(BridgeConfigInitializerComponent, BridgeConfigInitializer<Boostrap>)]
#[async_trait]
pub trait CanInitBridgeConfig:
    HasRuntimeType + HasChainDriverType + HasBridgeConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_bridge_config(
        &self,
        bridge_home_dir: &FilePathOf<Self::Runtime>,
        chain_driver: &Self::ChainDriver,
    ) -> Result<Self::BridgeConfig, Self::Error>;
}
