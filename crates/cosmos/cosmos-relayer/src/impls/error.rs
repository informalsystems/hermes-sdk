use core::convert::Infallible;
use core::num::ParseIntError;

use crate::types::error::{HandleInfallible, ProvideCosmosError};
use alloc::string::FromUtf8Error;
use cgp_core::prelude::*;
use cgp_core::{ErrorRaiser, ErrorTypeComponent};
use eyre::Report;
use hermes_cli_components::any_client::impls::decoders::client_state::UnknownClientStateType;
use hermes_cosmos_client_components::impls::queries::abci::AbciQueryError;
use hermes_protobuf_components::impls::any::TypeUrlMismatchError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::relay::impls::create_client::MissingCreateClientEventError;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_test_components::chain::impls::assert::poll_assert_eventual_amount::EventualAmountTimeoutError;
use hermes_test_components::chain::impls::ibc_transfer::MissingSendPacketEventError;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_types::clients::ics07_tendermint::error::Error as TendermintClientError;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::error::ValidationError as Ics24ValidationError;
use prost::{DecodeError, EncodeError};
use tendermint_proto::Error as TendermintProtoError;
use tendermint_rpc::Error as TendermintRpcError;

use crate::contexts::chain::CosmosChain;
use crate::types::error::{DebugError, DisplayError, Error, ReportError, ReturnError};

pub struct HandleCosmosError;

pub trait CheckErrorRaiser<Context>:
    ErrorRaiser<Context, TokioRuntimeError>
    + for<'a> ErrorRaiser<Context, &'a str>
    + for<'a> ErrorRaiser<Context, EventualAmountTimeoutError<'a, CosmosChain>>
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
        ErrorTypeComponent: ProvideCosmosError,
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TokioRuntimeError,
            RelayerError,
            SupervisorError,
            TendermintProtoError,
            TendermintRpcError,
            TendermintClientError,
            Ics02Error,
            Ics24ValidationError,
            ParseIntError,
            FromUtf8Error,
            EncodeError,
            DecodeError,
        ]: ReportError,
        [
            TypeUrlMismatchError,
            UnknownClientStateType,
            AbciQueryError,
            MissingSendPacketEventError,
        ]:
            DebugError,
    }
}

impl<'a> DelegateComponent<&'a str> for HandleCosmosError {
    type Delegate = DisplayError;
}

impl<'a, Chain> DelegateComponent<EventualAmountTimeoutError<'a, Chain>> for HandleCosmosError
where
    Chain: HasAddressType + HasAmountType,
{
    type Delegate = DebugError;
}

impl<'a, Chain, Counterparty>
    DelegateComponent<MissingCreateClientEventError<'a, Chain, Counterparty>> for HandleCosmosError
where
    Chain: HasChainIdType,
    Counterparty: HasChainIdType,
{
    type Delegate = DebugError;
}
