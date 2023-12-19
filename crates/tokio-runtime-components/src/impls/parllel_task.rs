use core::task::{Context, Poll};

use cgp_core::prelude::*;
use futures::stream::{Stream, StreamExt};
use futures::task::noop_waker;
use ibc_relayer_components::runtime::traits::task::{ConcurrentTaskRunner, Task};
use tokio::task::JoinSet;

use crate::impls::types::stream::HasBoxedStreamType;

pub struct TokioRunParallelTasks;

#[async_trait]
impl<Runtime, Components> ConcurrentTaskRunner<Runtime> for TokioRunParallelTasks
where
    Runtime: HasComponents<Components = Components>,
    Components: HasBoxedStreamType<Runtime>,
{
    async fn run_concurrent_tasks<T>(_runtime: &Runtime, tasks: Vec<T>)
    where
        T: Task,
    {
        run_parallel_tasks(tasks).await
    }

    async fn run_concurrent_task_stream<T>(_runtime: &Runtime, tasks: Components::Stream<T>)
    where
        T: Task,
    {
        run_parallel_task_stream(Components::to_boxed_stream(tasks)).await
    }
}

pub async fn run_parallel_tasks<T>(tasks: Vec<T>)
where
    T: Task,
{
    let mut join_set = JoinSet::new();

    for task in tasks.into_iter() {
        join_set.spawn(async move {
            task.run().await;
        });
    }

    while join_set.join_next().await.is_some() {}
}

pub async fn run_parallel_task_stream<T>(tasks: impl Stream<Item = T>)
where
    T: Task,
{
    let mut join_set = JoinSet::new();

    let waker = noop_waker();

    tasks
        .for_each_concurrent(None, |task| {
            join_set.spawn(async move {
                task.run().await;
            });

            let mut context = Context::from_waker(&waker);

            // Remove finished tasks from the JoinSet to avoid accumulation of
            // tasks from a potentially infinite stream.
            while let Poll::Ready(Some(_)) = join_set.poll_join_next(&mut context) {}

            async {}
        })
        .await;

    // Wait for all tasks to complete once the stream ends.
    while join_set.join_next().await.is_some() {}
}
