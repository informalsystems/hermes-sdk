use alloc::sync::Arc;

use cgp::prelude::*;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_tokio_runtime_components::components::parallel::*;
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
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
