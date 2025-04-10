use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

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
