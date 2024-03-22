use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;

#[derive_component(ChainNodeConfigInitializerComponent, ChainNodeConfigInitializer<Boostrap>)]
#[async_trait]
pub trait CanInitChainNodeConfig:
    HasChainNodeConfigType + HasChainType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn init_chain_node_config(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<Self::ChainNodeConfig, Self::Error>;
}
