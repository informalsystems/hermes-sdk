use alloc::boxed::Box;
use alloc::vec::Vec;

use hermes_prelude::*;

#[async_trait]
pub trait Task: Async {
    async fn run(self);
}

#[cgp_component {
  provider: ConcurrentTaskRunner,
  context: Runtime,
}]
#[async_trait]
pub trait CanRunConcurrentTasks: Async {
    async fn run_concurrent_tasks<T>(&self, tasks: Vec<Box<T>>)
    where
        T: Task;
}
