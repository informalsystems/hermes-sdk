use cgp_core::prelude::*;

use crate::traits::types::io::child_process::HasChildProcessType;
use crate::traits::types::io::file_path::HasFilePathType;

#[derive_component(ChainFullNodeStarterComponent, ChainFullNodeStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartChainFullNode: HasChildProcessType + HasFilePathType + HasErrorType {
    async fn start_chain_full_node(
        &self,
        chain_home_dir: &Self::FilePath,
    ) -> Result<Self::ChildProcess, Self::Error>;
}
