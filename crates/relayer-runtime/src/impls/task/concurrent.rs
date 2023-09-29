use futures::stream::Stream;
use futures::StreamExt;
use ibc_relayer_components::runtime::traits::task::Task;

pub async fn run_concurrent_tasks<T>(tasks: impl Stream<Item = T>)
where
    T: Task,
{
    tasks
        .for_each_concurrent(None, |task| Box::pin(async move { task.run().await }))
        .await;
}
