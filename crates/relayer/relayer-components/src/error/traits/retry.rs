use cgp::prelude::*;

#[cgp_component {
  name: RetryableErrorComponent,
  provider: ProvideRetryableError,
}]
pub trait HasRetryableError: HasErrorType {
    fn is_retryable_error(e: &Self::Error) -> bool;
}

#[cgp_component {
  name: MaxErrorRetryGetterComponent,
  provider: MaxErrorRetryGetter,
}]
pub trait HasMaxErrorRetry: Async {
    fn max_retry(&self) -> usize;
}
