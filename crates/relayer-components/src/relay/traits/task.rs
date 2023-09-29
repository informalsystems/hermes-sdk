use async_trait::async_trait;
use cgp_core::traits::Async;

use crate::std_prelude::*;

#[async_trait]
pub trait Task: Async {
    async fn run(&self);
}

#[async_trait]
pub trait CanRunConcurrentTasks<Task> {
    async fn run_concurrent_tasks(&self, tasks: Vec<Task>);
}
