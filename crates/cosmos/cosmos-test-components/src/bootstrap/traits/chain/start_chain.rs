use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;

#[derive_component(ChainFullNodeStarterComponent, ChainFullNodeStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartChainFullNode: HasChainNodeConfigType + HasRuntime + HasErrorType
where
    Self::Runtime: HasChildProcessType + HasFilePathType,
{
    async fn start_chain_full_node(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_node_config: &Self::ChainNodeConfig,
    ) -> Result<ChildProcessOf<Self::Runtime>, Self::Error>;
}
