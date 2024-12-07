use cgp::prelude::*;

use crate::traits::task::Task;

#[cgp_component {
  name: TaskSpawnerComponent,
  provider: TaskSpawner,
  context: Runtime,
}]
pub trait CanSpawnTask: Async {
    fn spawn_task<T>(&self, task: T)
    where
        T: Task;
}
