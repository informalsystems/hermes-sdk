use cgp::core::error::{ErrorTypeProviderComponent, HasAsyncErrorType};
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_relayer_components::error::traits::retry::{
    ProvideRetryableError, RetryableErrorComponent,
};

use crate::types::Error;

pub struct ProvideHermesError;

delegate_components! {
    ProvideHermesError {
        ErrorTypeProviderComponent: WithType<Error>
    }
}

#[cgp_provider(RetryableErrorComponent)]
impl<Context> ProvideRetryableError<Context> for ProvideHermesError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn is_retryable_error(e: &Error) -> bool {
        e.is_retryable
    }
}
