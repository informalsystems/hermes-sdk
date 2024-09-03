use cgp::core::Async;
use tokio::runtime::Runtime;

pub trait HasTokioRuntime: Async {
    fn tokio_runtime(&self) -> &Runtime;
}
