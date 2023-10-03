use cgp_async::async_trait;
use cgp_core::traits::Async;

use crate::runtime::traits::stream::HasStreamType;
use crate::std_prelude::*;

#[async_trait]
pub trait Task: Async {
    async fn run(self);
}

#[async_trait]
pub trait CanRunConcurrentTasks: HasStreamType {
    async fn run_concurrent_tasks<T>(&self, tasks: Vec<T>)
    where
        T: Task;

    async fn run_concurrent_task_stream<T>(&self, tasks: Self::Stream<T>)
    where
        T: Task;
}
