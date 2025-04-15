use cgp::prelude::*;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};

use crate::bootstrap::traits::HasChainGenesisConfigType;

#[cgp_component {
  provider: ChainGenesisConfigInitializer,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanInitChainGenesisConfig:
    HasRuntime + HasChainGenesisConfigType + HasAsyncErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn init_genesis_config(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
    ) -> Result<Self::ChainGenesisConfig, Self::Error>;
}
