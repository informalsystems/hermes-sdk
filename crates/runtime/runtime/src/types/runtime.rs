use alloc::sync::Arc;

use hermes_logging_components::traits::LoggerComponent;
use hermes_prelude::*;
use hermes_tokio_runtime_components::components::TokioParallelRuntimeComponents;
use hermes_tracing_logging_components::contexts::TracingLogger;
use tokio::runtime::Runtime;

#[cgp_context(HermesRuntimeComponents: TokioParallelRuntimeComponents)]
#[derive(Clone)]
pub struct HermesRuntime {
    pub runtime: Arc<Runtime>,
}

delegate_components! {
    HermesRuntimeComponents {
        LoggerComponent: TracingLogger,
    }
}

impl HermesRuntime {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self { runtime }
    }
}
