use cgp::prelude::*;

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
