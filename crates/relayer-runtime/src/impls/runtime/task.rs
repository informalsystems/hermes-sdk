use async_trait::async_trait;
use cgp_core::traits::Async;
use futures::stream::Stream;
use ibc_relayer_components::runtime::traits::task::{CanRunConcurrentTasks, Task};

use crate::types::runtime::TokioRuntimeContext;

#[async_trait]
impl CanRunConcurrentTasks for TokioRuntimeContext {
    async fn run_concurrent_tasks<S, T>(&self, tasks: S)
    where
        S: Stream<Item = T> + Async,
        T: Task,
    {
        self.run_concurrent_tasks(tasks).await
    }
}
