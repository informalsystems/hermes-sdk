use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

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
        bridge_home_dir: &FilePath<Self::Runtime>,
        chain_driver: &Self::ChainDriver,
    ) -> Result<Self::BridgeConfig, Self::Error>;
}
