use core::num::ParseIntError;
use std::string::FromUtf8Error;

use cgp_core::{Async, ErrorRaiser, HasErrorType, ProvideErrorType};
use eyre::eyre;
use hermes_cli_components::any_client::impls::decoders::client_state::UnknownClientStateType;
use hermes_cosmos_client_components::impls::decoders::type_url::TypeUrlMismatchError;
use hermes_cosmos_client_components::impls::queries::abci::AbciQueryError;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_test_components::chain::impls::ibc_transfer::MissingSendPacketEventError;
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::error::ValidationError as Ics24ValidationError;
use prost::{DecodeError, EncodeError};
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

impl<Context> ErrorRaiser<Context, ParseIntError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: ParseIntError) -> Error {
        BaseError::generic(e.into()).into()
    }
}

impl<Context> ErrorRaiser<Context, FromUtf8Error> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: FromUtf8Error) -> Error {
        BaseError::generic(e.into()).into()
    }
}

impl<Context> ErrorRaiser<Context, MissingSendPacketEventError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(_e: MissingSendPacketEventError) -> Error {
        BaseError::generic(eyre!("missing send packet event")).into()
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

impl<Context> ErrorRaiser<Context, Ics24ValidationError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(err: Ics24ValidationError) -> Error {
        BaseError::ics24_validation(err).into()
    }
}

impl<Context> ErrorRaiser<Context, TypeUrlMismatchError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: TypeUrlMismatchError) -> Error {
        BaseError::generic(eyre!(
            "type url mismatch. expected: {}, actual: {}",
            e.expected_url,
            e.actual_url
        ))
        .into()
    }
}

impl<Context> ErrorRaiser<Context, UnknownClientStateType> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: UnknownClientStateType) -> Error {
        BaseError::generic(eyre!("unknown client state type: {}", e.type_url,)).into()
    }
}

impl<Context> ErrorRaiser<Context, EncodeError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: EncodeError) -> Error {
        BaseError::generic(e.into()).into()
    }
}

impl<Context> ErrorRaiser<Context, DecodeError> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: DecodeError) -> Error {
        BaseError::generic(e.into()).into()
    }
}

impl<'a, Context> ErrorRaiser<Context, &'a str> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: &'a str) -> Error {
        BaseError::generic(eyre!("{e}")).into()
    }
}

impl<Context> ErrorRaiser<Context, eyre::Report> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn raise_error(e: eyre::Report) -> Error {
        BaseError::generic(e).into()
    }
}
