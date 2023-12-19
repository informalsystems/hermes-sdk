use cgp_core::prelude::*;
use futures::stream::{self, Stream, StreamExt};
use ibc_relayer_components::runtime::traits::task::{ConcurrentTaskRunner, Task};

use crate::impls::types::stream::HasBoxedStreamType;

pub struct RunConcurrentTasks;

#[async_trait]
impl<Runtime, Components> ConcurrentTaskRunner<Runtime> for RunConcurrentTasks
where
    Runtime: HasComponents<Components = Components>,
    Components: HasBoxedStreamType<Runtime>,
{
    async fn run_concurrent_tasks<T>(_runtime: &Runtime, tasks: Vec<T>)
    where
        T: Task,
    {
        run_concurrent_tasks(stream::iter(tasks)).await
    }

    async fn run_concurrent_task_stream<T>(_runtime: &Runtime, tasks: Components::Stream<T>)
    where
        T: Task,
    {
        run_concurrent_tasks(Components::to_boxed_stream(tasks)).await
    }
}
pub async fn run_concurrent_tasks<T>(tasks: impl Stream<Item = T>)
where
    T: Task,
{
    tasks
        .for_each_concurrent(None, |task| Box::pin(async move { task.run().await }))
        .await;
}
