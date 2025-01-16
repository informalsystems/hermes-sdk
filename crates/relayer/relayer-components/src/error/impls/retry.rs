use cgp::prelude::{Async, HasAsyncErrorType};

use crate::error::traits::retry::{MaxErrorRetryGetter, ProvideRetryableError};

pub struct ReturnRetryable<const RETRYABLE: bool>;

impl<Context, const RETRYABLE: bool> ProvideRetryableError<Context> for ReturnRetryable<RETRYABLE>
where
    Context: HasAsyncErrorType,
{
    fn is_retryable_error(_e: &Context::Error) -> bool {
        RETRYABLE
    }
}

pub struct ReturnMaxRetry<const MAX_RETRY: usize>;

impl<Context, const MAX_RETRY: usize> MaxErrorRetryGetter<Context> for ReturnMaxRetry<MAX_RETRY>
where
    Context: Async,
{
    fn max_retry(_context: &Context) -> usize {
        MAX_RETRY
    }
}
