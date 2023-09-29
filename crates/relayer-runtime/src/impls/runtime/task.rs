use async_trait::async_trait;
use ibc_relayer_components::relay::traits::task::{CanRunConcurrentTasks, Task};

use crate::types::runtime::TokioRuntimeContext;

#[async_trait]
impl<T> CanRunConcurrentTasks<T> for TokioRuntimeContext
where
    T: Task,
{
    async fn run_concurrent_tasks(&self, tasks: Vec<T>) {
        self.run_concurrent_tasks(tasks).await
    }
}
