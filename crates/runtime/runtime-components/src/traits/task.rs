use alloc::boxed::Box;
use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::stream::HasStreamType;

#[async_trait]
pub trait Task: Async {
    async fn run(self);
}

#[derive_component(ConcurrentTaskRunnerComponent, ConcurrentTaskRunner<Runtime>)]
#[async_trait]
pub trait CanRunConcurrentTasks: HasStreamType {
    async fn run_concurrent_tasks<T>(&self, tasks: Vec<Box<T>>)
    where
        T: Task;

    async fn run_concurrent_task_stream<T>(&self, tasks: Self::Stream<Box<T>>)
    where
        T: Task;
}
