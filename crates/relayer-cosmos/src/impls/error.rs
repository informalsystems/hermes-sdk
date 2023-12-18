use cgp_core::{Async, ErrorRaiser, HasErrorType, ProvideErrorType};
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_runtime::types::error::TokioRuntimeError;

use crate::types::error::{BaseError, Error};

pub struct HandleCosmosError;

impl<Context> ProvideErrorType<Context> for HandleCosmosError
where
    Context: Async,
{
    type Error = Error;
}

impl<Context> ErrorRaiser<Context, Error> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: Error) -> Error {
        e
    }
}

impl<Context> ErrorRaiser<Context, TokioRuntimeError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: TokioRuntimeError) -> Error {
        BaseError::tokio(e).into()
    }
}

impl<Context> ErrorRaiser<Context, RelayerError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(err: RelayerError) -> Error {
        BaseError::relayer(err).into()
    }
}

impl<Context> ErrorRaiser<Context, SupervisorError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(err: SupervisorError) -> Error {
        BaseError::supervisor(err).into()
    }
}

impl<Context> ErrorRaiser<Context, eyre::Report> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(err: eyre::Report) -> Error {
        BaseError::generic(err).into()
    }
}
