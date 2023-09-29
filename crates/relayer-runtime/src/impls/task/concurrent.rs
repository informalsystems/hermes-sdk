use futures::{stream, StreamExt};
use ibc_relayer_components::relay::traits::task::Task;

pub async fn run_concurrent_tasks<T>(tasks: Vec<T>)
where
    T: Task,
{
    stream::iter(tasks)
        .for_each_concurrent(None, |task| Box::pin(async move { task.run().await }))
        .await;
}
