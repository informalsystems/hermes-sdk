use core::pin::Pin;

use cgp_core::async_trait;
use futures::Future;
use hermes_relayer_components::runtime::traits::task::Task;

pub struct FutureTask {
    pub future: Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>,
}

impl FutureTask {
    pub fn new(future: impl Future<Output = ()> + Send + Sync + 'static) -> Self {
        Self {
            future: Box::pin(future),
        }
    }
}

#[async_trait]
impl Task for FutureTask {
    async fn run(self) {
        self.future.await;
    }
}
