use crate::traits::task::Task;
use cgp_core::prelude::*;

#[derive_component(TaskSpawnerComponent, TaskSpawner<Runtime>)]
pub trait CanSpawnTask: Async {
    fn spawn_task<T>(&self, task: T)
    where
        T: Task;
}
