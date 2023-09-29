use ibc_relayer_components::relay::traits::task::Task;
use tokio::task::JoinSet;

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

    while let Some(_) = join_set.join_next().await {}
}
