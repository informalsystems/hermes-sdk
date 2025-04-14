use alloc::boxed::Box;
use alloc::vec::Vec;

use cgp::prelude::*;
use futures_util::stream::{self, Stream, StreamExt};
use hermes_runtime_components::traits::task::{
    ConcurrentTaskRunner, ConcurrentTaskRunnerComponent, Task,
};

#[cgp_new_provider(ConcurrentTaskRunnerComponent)]
impl<Runtime> ConcurrentTaskRunner<Runtime> for RunConcurrentTasks
where
    Runtime: Async,
{
    async fn run_concurrent_tasks<T>(_runtime: &Runtime, tasks: Vec<Box<T>>)
    where
        T: Task,
    {
        run_concurrent_tasks(stream::iter(tasks)).await
    }
}

pub async fn run_concurrent_tasks<T>(tasks: impl Stream<Item = Box<T>>)
where
    T: Task,
{
    tasks
        .for_each_concurrent(None, |task| Box::pin(async move { task.run().await }))
        .await;
}
