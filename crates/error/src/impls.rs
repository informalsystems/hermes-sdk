use cgp::core::error::{ErrorTypeProviderComponent, ErrorWrapperComponent, HasAsyncErrorType};
use cgp::core::types::WithType;
use cgp::prelude::*;
use hermes_relayer_components::error::traits::{ProvideRetryableError, RetryableErrorComponent};

use crate::handlers::WrapErrorDetail;
use crate::types::Error;

pub struct UseHermesError;

delegate_components! {
    UseHermesError {
        ErrorTypeProviderComponent: WithType<Error>,
        ErrorWrapperComponent: WrapErrorDetail,
    }
}

#[cgp_provider(RetryableErrorComponent)]
impl<Context> ProvideRetryableError<Context> for UseHermesError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn is_retryable_error(e: &Error) -> bool {
        e.is_retryable
    }
}
