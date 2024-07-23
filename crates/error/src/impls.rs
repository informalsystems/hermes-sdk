use cgp_core::error::{HasErrorType, ProvideErrorType};
use cgp_core::Async;
use hermes_relayer_components::error::traits::retry::ProvideRetryableError;

use crate::types::Error;

pub struct ProvideHermesError;

impl<Context> ProvideErrorType<Context> for ProvideHermesError
where
    Context: Async,
{
    type Error = Error;
}

impl<Context> ProvideRetryableError<Context> for ProvideHermesError
where
    Context: HasErrorType<Error = Error>,
{
    fn is_retryable_error(e: &Error) -> bool {
        e.is_retryable
    }
}
