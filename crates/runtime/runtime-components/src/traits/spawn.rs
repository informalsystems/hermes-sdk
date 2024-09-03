use cgp::prelude::*;

use crate::traits::task::Task;

#[derive_component(TaskSpawnerComponent, TaskSpawner<Runtime>)]
pub trait CanSpawnTask: Async {
    fn spawn_task<T>(&self, task: T)
    where
        T: Task;
}
