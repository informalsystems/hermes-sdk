use alloc::sync::Arc;

use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct HermesRuntime {
    pub runtime: Arc<Runtime>,
}

impl HermesRuntime {
    pub fn new(runtime: Arc<Runtime>) -> Self {
        Self { runtime }
    }
}
