use cgp::prelude::*;
use hermes_runtime_components::traits::{Task, TaskSpawner, TaskSpawnerComponent};

use crate::traits::HasTokioRuntime;

pub struct TokioSpawnTask;

#[cgp_provider(TaskSpawnerComponent)]
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
