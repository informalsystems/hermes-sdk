use cgp::core::error::{ErrorTypeComponent, HasErrorType};
use cgp::core::types::impls::WithType;
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
    Context: HasErrorType<Error = Error>,
{
    fn is_retryable_error(e: &Error) -> bool {
        e.is_retryable
    }
}
