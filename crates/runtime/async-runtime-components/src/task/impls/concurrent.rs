use alloc::boxed::Box;
use alloc::vec::Vec;

use futures_util::stream::{self, Stream, StreamExt};
use hermes_runtime_components::traits::task::{ConcurrentTaskRunner, Task};

use crate::stream::traits::boxed::HasBoxedStreamType;

pub struct RunConcurrentTasks;

impl<Runtime> ConcurrentTaskRunner<Runtime> for RunConcurrentTasks
where
    Runtime: HasBoxedStreamType,
{
    async fn run_concurrent_tasks<T>(_runtime: &Runtime, tasks: Vec<Box<T>>)
    where
        T: Task,
    {
        run_concurrent_tasks(stream::iter(tasks)).await
    }

    async fn run_concurrent_task_stream<T>(_runtime: &Runtime, tasks: Runtime::Stream<Box<T>>)
    where
        T: Task,
    {
        run_concurrent_tasks(Runtime::to_boxed_stream(tasks)).await
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
