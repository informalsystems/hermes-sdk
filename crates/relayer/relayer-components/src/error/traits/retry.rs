use cgp::prelude::*;

#[cgp_component {
    provider: RetryPerformer,
}]
#[async_trait]
pub trait CanPerformRetry: HasAsyncErrorType {
    async fn perform_retry<T: Async>(
        num_retries: usize,
        cont: impl AsyncFn() -> Result<T, Self::Error> + Send,
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
