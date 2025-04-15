use alloc::boxed::Box;
use core::future::Future;
use core::pin::Pin;

use hermes_runtime_components::traits::Task;

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

impl Task for FutureTask {
    async fn run(self) {
        self.future.await;
    }
}
