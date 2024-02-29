use core::num::ParseIntError;

use alloc::string::FromUtf8Error;
use cgp_core::prelude::*;
use cgp_core::{ErrorRaiser, ProvideErrorType};
use eyre::Report;
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

use crate::types::error2::{Error, MessageNonRetryableError, ReportNonRetryableError, ReturnError};

pub struct HandleCosmosError;

impl<Context> ProvideErrorType<Context> for HandleCosmosError
where
    Context: Async,
{
    type Error = Error;
}

pub trait CheckErrorRaiser<Context>:
    ErrorRaiser<Context, TokioRuntimeError> + for<'a> ErrorRaiser<Context, &'a str>
where
    Context: HasErrorType<Error = Error>,
{
}

impl<Context> CheckErrorRaiser<Context> for HandleCosmosError where
    Context: HasErrorType<Error = Error>
{
}

impl<Context, E, Delegate> ErrorRaiser<Context, E> for HandleCosmosError
where
    Context: HasErrorType,
    Self: DelegateComponent<E, Delegate = Delegate>,
    Delegate: ErrorRaiser<Context, E>,
{
    fn raise_error(e: E) -> Context::Error {
        Delegate::raise_error(e)
    }
}

delegate_components! {
    HandleCosmosError {
        Error: ReturnError,
        [
            Report,
            TokioRuntimeError,
            RelayerError,
            SupervisorError,
            TendermintProtoError,
            TendermintRpcError,
            Ics02Error,
            Ics24ValidationError,
            TypeUrlMismatchError,
            AbciQueryError,
            MissingSendPacketEventError,
            UnknownClientStateType,
            ParseIntError,
            FromUtf8Error,
            EncodeError,
            DecodeError,
        ]: ReportNonRetryableError,
    }
}

impl<'a> DelegateComponent<&'a str> for HandleCosmosError {
    type Delegate = MessageNonRetryableError;
}
