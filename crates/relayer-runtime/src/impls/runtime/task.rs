use async_trait::async_trait;
use cgp_core::traits::Async;
use futures::stream::Stream;
use ibc_relayer_components::runtime::traits::task::{CanRunConcurrentTasks, Task};

use crate::impls::task::parallel::run_parallel_tasks;
use crate::types::runtime::TokioRuntimeContext;

#[async_trait]
impl CanRunConcurrentTasks for TokioRuntimeContext {
    async fn run_concurrent_tasks<S, T>(&self, tasks: S)
    where
        S: Stream<Item = T> + Async,
        T: Task,
    {
        run_parallel_tasks(tasks).await
    }
}
