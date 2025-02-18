use cgp::core::error::{ErrorRaiser, ErrorRaiserComponent};
use cgp::prelude::*;

pub struct MaxRetryExceededError<'a, Context>
where
    Context: HasAsyncErrorType,
{
    pub context: &'a Context,
    pub error: Context::Error,
    pub max_retry: usize,
}

pub struct UnwrapMaxRetryExceededError;

#[cgp_provider(ErrorRaiserComponent)]
impl<'a, Context> ErrorRaiser<Context, MaxRetryExceededError<'a, Context>>
    for UnwrapMaxRetryExceededError
where
    Context: HasAsyncErrorType,
{
    fn raise_error(e: MaxRetryExceededError<'a, Context>) -> Context::Error {
        e.error
    }
}
