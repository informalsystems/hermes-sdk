use hermes_runtime_components::traits::spawn::TaskSpawner;
use hermes_runtime_components::traits::task::Task;

use crate::traits::runtime::HasTokioRuntime;

pub struct TokioSpawnTask;

impl<Runtime> TaskSpawner<Runtime> for TokioSpawnTask
where
    Runtime: HasTokioRuntime,
{
    fn spawn_task<T>(runtime: &Runtime, task: T)
    where
        T: Task,
    {
        runtime.tokio_runtime().spawn(async move {
            task.run().await;
        });
    }
}
