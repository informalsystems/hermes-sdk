use core::future::Future;

use ibc_relayer_components_extra::runtime::traits::spawn::{CanSpawnTask, HasSpawner, Spawner};

use crate::types::runtime::TokioRuntimeContext;

impl CanSpawnTask for TokioRuntimeContext {
    fn spawn<T>(&self, task: T)
    where
        T: ibc_relayer_components::runtime::traits::task::Task,
    {
        self.runtime.spawn(async move {
            task.run().await;
        });
    }
}

impl HasSpawner for TokioRuntimeContext {
    type Spawner = Self;

    fn spawner(&self) -> Self::Spawner {
        self.clone()
    }
}

impl Spawner for TokioRuntimeContext {
    fn spawn<F>(&self, task: F)
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        self.runtime.spawn(async move {
            task.await;
        });
    }
}
