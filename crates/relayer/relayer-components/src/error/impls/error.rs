use cgp_core::{ErrorRaiser, HasErrorType};

pub struct MaxRetryExceededError<'a, Context>
where
    Context: HasErrorType,
{
    pub context: &'a Context,
    pub error: Context::Error,
    pub max_retry: usize,
}

pub struct UnwrapMaxRetryExceededError;

impl<'a, Context> ErrorRaiser<Context, MaxRetryExceededError<'a, Context>>
    for UnwrapMaxRetryExceededError
where
    Context: HasErrorType,
{
    fn raise_error(e: MaxRetryExceededError<'a, Context>) -> Context::Error {
        e.error
    }
}
