use cgp::prelude::*;
use hermes_runtime_components::traits::os::child_process::{ChildProcessOf, HasChildProcessType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[derive_component(ChainProcessTakerComponent, ChainProcessTaker<ChainDriver>)]
#[async_trait]
pub trait CanTakeChainProcess: HasRuntime<Runtime: HasChildProcessType> {
    async fn take_chain_process(&self) -> Option<ChildProcessOf<Self::Runtime>>;
}
