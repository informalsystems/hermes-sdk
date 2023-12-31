use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

use crate::bootstrap::traits::types::chain_config::HasChainConfigType;

#[derive_component(ChainFullNodeStarterComponent, ChainFullNodeStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartChainFullNode: HasChainConfigType + HasRuntime + HasErrorType
where
    Self::Runtime: HasChildProcessType + HasFilePathType,
{
    async fn start_chain_full_node(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_config: &Self::ChainConfig,
    ) -> Result<ChildProcess<Self::Runtime>, Self::Error>;
}
