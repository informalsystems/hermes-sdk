use cgp::prelude::*;

#[derive_component(RetryableErrorComponent, ProvideRetryableError<Context>)]
pub trait HasRetryableError: HasErrorType {
    fn is_retryable_error(e: &Self::Error) -> bool;
}

#[derive_component(MaxErrorRetryGetterComponent, MaxErrorRetryGetter<Context>)]
pub trait HasMaxErrorRetry: Async {
    fn max_retry(&self) -> usize;
}
