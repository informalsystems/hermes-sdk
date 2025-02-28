use cgp::prelude::*;

use crate::error::traits::{ProvideRetryableError, RetryableErrorComponent};

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
