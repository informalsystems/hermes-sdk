use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;

#[derive_component(ChainNodeConfigInitializerComponent, ChainNodeConfigInitializer<Boostrap>)]
#[async_trait]
pub trait CanInitChainNodeConfig: HasChainNodeConfigType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_chain_node_config(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<Self::ChainNodeConfig, Self::Error>;
}
