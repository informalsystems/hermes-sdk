use async_trait::async_trait;
use cgp_core::traits::Async;
use futures::stream::Stream;
use ibc_relayer_components::runtime::traits::task::{CanRunConcurrentTasks, Task};

use crate::types::runtime::TokioRuntimeContext;

#[async_trait]
impl<T> CanRunConcurrentTasks<T> for TokioRuntimeContext
where
    T: Task,
{
    async fn run_concurrent_tasks(&self, tasks: impl Stream<Item = T> + Async) {
        self.run_concurrent_tasks(tasks).await
    }
}
