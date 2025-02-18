use cgp::prelude::*;

use crate::error::traits::retry::{
    MaxErrorRetryGetter, MaxErrorRetryGetterComponent, ProvideRetryableError,
    RetryableErrorComponent,
};

pub struct ReturnRetryable<const RETRYABLE: bool>;

#[cgp_provider(RetryableErrorComponent)]
impl<Context, const RETRYABLE: bool> ProvideRetryableError<Context> for ReturnRetryable<RETRYABLE>
where
    Context: HasAsyncErrorType,
{
    fn is_retryable_error(_e: &Context::Error) -> bool {
        RETRYABLE
    }
}

pub struct ReturnMaxRetry<const MAX_RETRY: usize>;

#[cgp_provider(MaxErrorRetryGetterComponent)]
impl<Context, const MAX_RETRY: usize> MaxErrorRetryGetter<Context> for ReturnMaxRetry<MAX_RETRY>
where
    Context: Async,
{
    fn max_retry(_context: &Context) -> usize {
        MAX_RETRY
    }
}
