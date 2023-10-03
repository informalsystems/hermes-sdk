use core::future::Future;

use cgp_core::traits::Async;
use ibc_relayer_components::runtime::traits::task::Task;

use crate::std_prelude::*;

pub trait CanSpawnTask: Async {
    fn spawn<T>(&self, task: T)
    where
        T: Task;
}

pub trait HasSpawner: Async {
    type Spawner: Spawner;

    fn spawner(&self) -> Self::Spawner;
}

pub trait Spawner: Async {
    fn spawn<F>(&self, task: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static;
}
