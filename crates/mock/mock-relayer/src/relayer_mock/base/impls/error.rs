use cgp_core::{Async, ErrorRaiser, HasErrorType, ProvideErrorType};
use hermes_runtime::types::error::TokioRuntimeError;

use crate::relayer_mock::base::error::{BaseError, Error};

pub struct HandleMockError;

impl<Context> ProvideErrorType<Context> for HandleMockError
where
    Context: Async,
{
    type Error = Error;
}

impl<Context> ErrorRaiser<Context, Error> for HandleMockError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

impl<Context> ErrorRaiser<Context, TokioRuntimeError> for HandleMockError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}
