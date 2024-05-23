use cgp_core::prelude::*;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::types::rollup_node_config::HasRollupNodeConfigType;

#[derive_component(RollupNodeConfigInitializerComponent, RollupNodeConfigInitializer<Bootstrap>)]
#[async_trait]
pub trait CanInitRollupNodeConfig:
    HasRuntimeType + HasChainType + HasBridgeDriverType + HasRollupNodeConfigType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasAddressType,
{
    async fn init_rollup_node_config(
        &self,
        rollup_home_dir: &FilePathOf<Self::Runtime>,
        bridge_driver: &Self::BridgeDriver,
        sequencer_da_address: &AddressOf<Self::Chain>,
    ) -> Result<Self::RollupNodeConfig, Self::Error>;
}
