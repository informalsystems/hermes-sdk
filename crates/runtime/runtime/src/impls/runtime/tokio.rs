use hermes_tokio_runtime_components::traits::HasTokioRuntime;
use tokio::runtime::Runtime;

use crate::types::runtime::HermesRuntime;

impl HasTokioRuntime for HermesRuntime {
    fn tokio_runtime(&self) -> &Runtime {
        &self.runtime
    }
}
