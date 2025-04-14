use cgp::prelude::*;
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

#[cgp_component {
  provider: ChainNodeConfigInitializer,
  context: Boostrap,
}]
#[async_trait]
pub trait CanInitChainNodeConfig:
    HasChainNodeConfigType
    + HasChainGenesisConfigType
    + HasChainType
    + HasRuntimeType
    + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasChainIdType,
{
    async fn init_chain_node_config(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_id: &ChainIdOf<Self::Chain>,
        genesis_config: &Self::ChainGenesisConfig,
    ) -> Result<Self::ChainNodeConfig, Self::Error>;
}
