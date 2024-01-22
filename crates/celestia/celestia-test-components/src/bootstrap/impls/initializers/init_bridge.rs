use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[async_trait]
pub trait CanInitCelestiaBridge: HasChainType + HasRuntime + HasErrorType
where
    Self::Chain: HasChainIdType,
    Self::Runtime: HasFilePathType + HasChildProcessType,
{
    async fn init_celestia_bridge(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &ChainId<Self::Chain>,
    ) -> Result<ChildProcess<Self::Runtime>, Self::Error>;
}

impl<Bootstrap, Chain, Runtime> CanInitCelestiaBridge for Bootstrap
where
    Bootstrap: HasChainType<Chain = Chain> + HasRuntime<Runtime = Runtime> + HasErrorType,
    Chain: HasChainIdType,
    Runtime: HasFilePathType + HasChildProcessType,
{
    async fn init_celestia_bridge(
        &self,
        _chain_home_dir: &Runtime::FilePath,
        _chain_id: &Chain::ChainId,
    ) -> Result<Runtime::ChildProcess, Self::Error> {
        todo!()
    }
}
