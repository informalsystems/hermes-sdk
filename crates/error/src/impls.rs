use cgp::core::error::{ErrorTypeComponent, HasAsyncErrorType};
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_relayer_components::error::traits::retry::ProvideRetryableError;

use crate::types::Error;

pub struct ProvideHermesError;

delegate_components! {
    ProvideHermesError {
        ErrorTypeComponent: WithType<Error>
    }
}

impl<Context> ProvideRetryableError<Context> for ProvideHermesError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn is_retryable_error(e: &Error) -> bool {
        e.is_retryable
    }
}
