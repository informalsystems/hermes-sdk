use cgp::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorTypeProvider, ErrorTypeProviderComponent,
};
use cgp::prelude::*;
use eyre::eyre;
use hermes_relayer_components::chain::traits::send_message::EmptyMessageResponse;
use hermes_runtime::types::error::TokioRuntimeError;

use crate::relayer_mock::base::error::{BaseError, Error};

pub struct HandleMockError;

#[cgp_provider(ErrorTypeProviderComponent)]
impl<Context> ErrorTypeProvider<Context> for HandleMockError
where
    Context: Async,
{
    type Error = Error;
}

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, Error> for HandleMockError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, EmptyMessageResponse> for HandleMockError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(_e: EmptyMessageResponse) -> Error {
        BaseError::generic(eyre!("empty message response")).into()
    }
}

#[cgp_provider(ErrorRaiserComponent)]
impl<Context> ErrorRaiser<Context, TokioRuntimeError> for HandleMockError
where
    Context: HasAsyncErrorType<Error = Error>,
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}
