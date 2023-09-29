use async_trait::async_trait;
use cgp_core::traits::Async;
use futures_core::Stream;

use crate::std_prelude::*;

#[async_trait]
pub trait Task: Async {
    async fn run(&self);
}

#[async_trait]
pub trait CanRunConcurrentTasks {
    async fn run_concurrent_tasks<S, T>(&self, tasks: S)
    where
        S: Stream<Item = T> + Async,
        T: Task;
}
