use cgp_core::{Async, ErrorRaiser, HasErrorType, ProvideErrorType};
use eyre::eyre;
use hermes_cosmos_client_components::impls::queries::abci::AbciQueryError;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use tendermint_proto::Error as TendermintProtoError;
use tendermint_rpc::Error as TendermintRpcError;

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

impl<Context> ErrorRaiser<Context, TendermintProtoError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: TendermintProtoError) -> Error {
        BaseError::generic(e.into()).into()
    }
}

impl<Context> ErrorRaiser<Context, AbciQueryError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: AbciQueryError) -> Error {
        BaseError::generic(eyre!("abci query returned error: {:?}", e.response)).into()
    }
}

impl<Context> ErrorRaiser<Context, TendermintRpcError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(err: TendermintRpcError) -> Error {
        BaseError::tendermint_rpc(err).into()
    }
}

impl<Context> ErrorRaiser<Context, Ics02Error> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(err: Ics02Error) -> Error {
        BaseError::ics02(err).into()
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
