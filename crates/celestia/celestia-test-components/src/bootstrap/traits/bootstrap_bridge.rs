use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};
#[derive_component(BridgeBootstrapperComponent, BridgeBootstrapper<Bootstrap>)]
#[async_trait]
pub trait CanBootstrapBridge: HasChainDriverType + HasRuntime + HasErrorType
where
    Self::Chain: HasChainIdType,
    Self::Runtime: HasChildProcessType,
{
    async fn bootstrap_bridge(
        &self,
        chain_driver: &Self::ChainDriver,
    ) -> Result<ChildProcess<Self::Runtime>, Self::Error>;
}
