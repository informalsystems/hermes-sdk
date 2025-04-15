use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_runtime_components::traits::{ChildProcessOf, HasChildProcessType, HasRuntime};

#[cgp_component {
  provider: ChainProcessTaker,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanTakeChainProcess: HasRuntime<Runtime: HasChildProcessType> {
    fn take_chain_process(&mut self) -> Vec<ChildProcessOf<Self::Runtime>>;
}
