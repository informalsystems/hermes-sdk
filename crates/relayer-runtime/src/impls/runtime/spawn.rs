use core::future::Future;

use ibc_relayer_components_extra::runtime::traits::spawn::{HasSpawner, Spawner};

use crate::types::runtime::TokioRuntimeContext;

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
