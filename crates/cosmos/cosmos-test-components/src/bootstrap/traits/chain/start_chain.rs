use cgp::prelude::*;
use hermes_runtime_components::traits::{
    ChildProcessOf, FilePathOf, HasChildProcessType, HasFilePathType, HasRuntime,
};

use crate::bootstrap::traits::{HasChainGenesisConfigType, HasChainNodeConfigType};

#[cgp_component {
  provider: ChainFullNodeStarter,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanStartChainFullNodes:
    HasChainNodeConfigType
    + HasChainGenesisConfigType
    + HasRuntime<Runtime: HasChildProcessType + HasFilePathType>
    + HasAsyncErrorType
{
    async fn start_chain_full_nodes(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_node_config: &Self::ChainNodeConfig,
        chain_genesis_config: &Self::ChainGenesisConfig,
    ) -> Result<Vec<ChildProcessOf<Self::Runtime>>, Self::Error>;
}
