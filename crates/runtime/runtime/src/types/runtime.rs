use alloc::sync::Arc;

use cgp::prelude::*;
use hermes_tokio_runtime_components::components::parallel::*;
use tokio::runtime::Runtime;

#[cgp_context(HermesRuntimeComponents: TokioParallelRuntimeComponents)]
#[derive(Clone)]
pub struct HermesRuntime {
    pub runtime: Arc<Runtime>,
}

impl HermesRuntime {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self { runtime }
    }
}
