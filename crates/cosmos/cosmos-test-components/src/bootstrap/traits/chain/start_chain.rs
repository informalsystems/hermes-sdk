use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcessOf, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;

#[derive_component(ChainFullNodeStarterComponent, ChainFullNodeStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartChainFullNode: HasChainNodeConfigType + HasRuntime + HasErrorType
where
    Self::Runtime: HasChildProcessType + HasFilePathType,
{
    async fn start_chain_full_nodes(
        &self,
        chain_home_dir: &FilePathOf<Self::Runtime>,
        chain_node_config: &Self::ChainNodeConfig,
    ) -> Result<Vec<ChildProcessOf<Self::Runtime>>, Self::Error>;
}
