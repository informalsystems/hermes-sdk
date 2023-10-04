use core::pin::Pin;

use async_trait::async_trait;
use futures::stream::Stream;
use ibc_relayer_components::runtime::traits::task::{CanRunConcurrentTasks, Task};

use crate::impls::task::parallel::{run_parallel_task_stream, run_parallel_tasks};
use crate::types::runtime::TokioRuntimeContext;

#[async_trait]
impl CanRunConcurrentTasks for TokioRuntimeContext {
    async fn run_concurrent_tasks<T>(&self, tasks: Vec<T>)
    where
        T: Task,
    {
        run_parallel_tasks(tasks).await
    }

    async fn run_concurrent_task_stream<T>(
        &self,
        tasks: Pin<Box<dyn Stream<Item = T> + Send + Sync + 'static>>,
    ) where
        T: Task,
    {
        run_parallel_task_stream(tasks).await
    }
}
