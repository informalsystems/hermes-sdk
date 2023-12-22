use cgp_core::Async;
use tokio::runtime::Runtime;

pub trait HasTokioRuntime: Async {
    fn tokio_runtime(&self) -> &Runtime;
}
