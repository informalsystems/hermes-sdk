use ibc_relayer_components::runtime::traits::task::Task;
use ibc_relayer_components_extra::runtime::traits::spawn::TaskSpawner;

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
