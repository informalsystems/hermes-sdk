use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use ibc_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(ChainFullNodeStarterComponent, ChainFullNodeStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartChainFullNode: HasRuntime + HasErrorType
where
    Self::Runtime: HasChildProcessType + HasFilePathType,
{
    async fn start_chain_full_node(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
    ) -> Result<ChildProcess<Self::Runtime>, Self::Error>;
}
