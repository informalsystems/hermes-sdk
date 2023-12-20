use tokio::runtime::Runtime;
use tokio_runtime_components::traits::runtime::HasTokioRuntime;

use crate::types::runtime::HermesRuntime;

impl HasTokioRuntime for HermesRuntime {
    fn tokio_runtime(&self) -> &Runtime {
        &self.runtime
    }
}
