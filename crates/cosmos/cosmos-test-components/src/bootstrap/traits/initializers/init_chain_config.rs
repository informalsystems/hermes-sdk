use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::relayer_components::chain::types::aliases::ChainIdOf;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntimeType};
use hermes_core::test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::{HasChainGenesisConfigType, HasChainNodeConfigType};

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
