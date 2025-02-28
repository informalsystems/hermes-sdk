use core::future::Future;

use cgp::prelude::*;

#[cgp_component {
    provider: RetryPerformer,
}]
#[async_trait]
pub trait CanPerformRetry: HasAsyncErrorType {
    async fn perform_with_retry<T: Send + Sync>(
        &self,
        task_name: &str,
        num_retries: usize,
        cont: impl AsyncCont<Result<T, Self::Error>>,
    ) -> Result<T, Self::Error>;
}

#[cgp_component {
    name: RetryableErrorComponent,
    provider: ProvideRetryableError,
}]
pub trait HasRetryableError: HasAsyncErrorType {
    fn is_retryable_error(e: &Self::Error) -> bool;
}

#[cgp_component {
  provider: MaxErrorRetryGetter,
}]
pub trait HasMaxErrorRetry: Async {
    fn max_retry(&self) -> usize;
}

#[async_trait]
pub trait AsyncCont<T: Send + Sync>: Send + Sync {
    async fn run(&self) -> T;
}

impl<F, T, Fut> AsyncCont<T> for F
where
    T: Send + Sync,
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = T> + Send,
{
    async fn run(&self) -> T {
        self().await
    }
}
