use tokio::runtime::Runtime;
use tokio_runtime_components::traits::runtime::HasTokioRuntime;

use crate::types::runtime::TokioRuntimeContext;

impl HasTokioRuntime for TokioRuntimeContext {
    fn tokio_runtime(&self) -> &Runtime {
        &self.runtime
    }
}
