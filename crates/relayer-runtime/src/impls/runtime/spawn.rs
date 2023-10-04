use ibc_relayer_components_extra::runtime::traits::spawn::CanSpawnTask;

use crate::types::runtime::TokioRuntimeContext;

impl CanSpawnTask for TokioRuntimeContext {
    fn spawn_task<T>(&self, task: T)
    where
        T: ibc_relayer_components::runtime::traits::task::Task,
    {
        self.runtime.spawn(async move {
            task.run().await;
        });
    }
}
