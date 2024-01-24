use cgp_core::prelude::*;
use hermes_cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(BridgeBootstrapperComponent, BridgeBootstrapper<Bootstrap>)]
#[async_trait]
pub trait CanBootstrapBridge: HasChainType + HasRuntime + HasErrorType
where
    Self::Chain: HasChainIdType,
    Self::Runtime: HasFilePathType + HasChildProcessType,
{
    async fn bootstrap_bridge(
        &self,
        chain: &Self::Chain,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_config: &CosmosChainConfig,
    ) -> Result<ChildProcess<Self::Runtime>, Self::Error>;
}
