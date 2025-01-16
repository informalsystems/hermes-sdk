use cgp::core::error::{ErrorRaiser, HasAsyncErrorType, ProvideErrorType};
use cgp::core::Async;
use eyre::eyre;
use hermes_relayer_components::chain::traits::send_message::EmptyMessageResponse;
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
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

impl<Context> ErrorRaiser<Context, EmptyMessageResponse> for HandleMockError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(_e: EmptyMessageResponse) -> Error {
        BaseError::generic(eyre!("empty message response")).into()
    }
}

impl<Context> ErrorRaiser<Context, TokioRuntimeError> for HandleMockError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}
