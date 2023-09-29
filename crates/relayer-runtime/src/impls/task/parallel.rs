use futures::stream::{Stream, StreamExt};
use ibc_relayer_components::runtime::traits::task::Task;
use tokio::task::JoinSet;

pub async fn run_parallel_tasks<T>(tasks: impl Stream<Item = T>)
where
    T: Task,
{
    let mut join_set = JoinSet::new();

    tasks
        .for_each(|task| {
            join_set.spawn(async move {
                task.run().await;
            });

            async {}
        })
        .await;

    while let Some(_) = join_set.join_next().await {}
}
