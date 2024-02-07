use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcessOf, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(BridgeStarterComponent, BridgeStarter<Bootstrap>)]
#[async_trait]
pub trait CanStartBridge: HasChainDriverType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasChildProcessType + HasFilePathType,
{
    async fn start_bridge(
        &self,
        bridge_home_dir: &FilePathOf<Self::Runtime>,
        chain_driver: &Self::ChainDriver,
    ) -> Result<ChildProcessOf<Self::Runtime>, Self::Error>;
}
