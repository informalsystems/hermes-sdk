use alloc::string::FromUtf8Error;
use core::convert::Infallible;
use core::num::ParseIntError;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use eyre::Report;
use hermes_cli_components::any_client::impls::encoding::encode::UnknownClientStateType;
use hermes_cosmos_chain_components::impls::queries::abci::AbciQueryError;
use hermes_cosmos_chain_components::impls::transaction::submit_tx::BroadcastTxError;
use hermes_protobuf_encoding_components::impls::any::TypeUrlMismatchError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::error::impls::delegate::DelegateErrorRaiser;
use hermes_relayer_components::error::impls::error::{
    MaxRetryExceededError, UnwrapMaxRetryExceededError,
};
use hermes_relayer_components::error::traits::retry::ProvideRetryableError;
use hermes_relayer_components::relay::impls::create_client::MissingCreateClientEventError;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_relayer_runtime::types::error::TokioRuntimeError;
use hermes_test_components::chain::impls::assert::poll_assert_eventual_amount::EventualAmountTimeoutError;
use hermes_test_components::chain::impls::ibc_transfer::MissingSendPacketEventError;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::keyring::errors::Error as KeyringError;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_types::clients::ics07_tendermint::error::Error as TendermintClientError;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics24_host::error::ValidationError as Ics24ValidationError;
use ibc_relayer_types::signer::SignerError;
use prost::{DecodeError, EncodeError};
use tendermint_proto::Error as TendermintProtoError;
use tendermint_rpc::Error as TendermintRpcError;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::contexts::chain::CosmosChain;
use crate::types::error::{
    DebugError, DisplayError, Error, HandleInfallible, ProvideCosmosError, ReportError, ReturnError,
};

pub struct HandleCosmosError;

pub struct CosmosErrorHandlers;

pub trait CanHandleCosmosError<Context>:
    ErrorRaiser<Context, TokioRuntimeError>
    + ErrorRaiser<Context, Status>
    + ErrorRaiser<Context, TransportError>
    + for<'a> ErrorRaiser<Context, &'a str>
    + for<'a> ErrorRaiser<Context, EventualAmountTimeoutError<'a, CosmosChain>>
    + for<'a> ErrorRaiser<Context, BroadcastTxError<'a, CosmosChain>>
    + for<'a> ErrorRaiser<Context, TxNoResponseError<'a, CosmosChain>>
    + for<'a> ErrorRaiser<Context, MaxRetryExceededError<'a, Context>>
where
    Context: HasErrorType<Error = Error>,
{
}

impl<Context> CanHandleCosmosError<Context> for HandleCosmosError where
    Context: HasErrorType<Error = Error>
{
}

impl<Context> ProvideRetryableError<Context> for HandleCosmosError
where
    Context: HasErrorType<Error = Error>,
{
    fn is_retryable_error(e: &Error) -> bool {
        e.is_retryable
    }
}

delegate_components! {
    HandleCosmosError {
        ErrorTypeComponent: ProvideCosmosError,
        ErrorRaiserComponent:
            DelegateErrorRaiser<CosmosErrorHandlers>,
    }
}

delegate_components! {
    CosmosErrorHandlers {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TokioRuntimeError,
            RelayerError,
            SignerError,
            KeyringError,
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

            // TODO: make it retryable?
            TransportError,
        ]: ReportError,
        [
            TypeUrlMismatchError,
            UnknownClientStateType,
            AbciQueryError,
            MissingSendPacketEventError,
            Status,
        ]:
            DebugError,
    }
}

impl<'a> DelegateComponent<&'a str> for CosmosErrorHandlers {
    type Delegate = DisplayError;
}

impl<'a, Chain> DelegateComponent<EventualAmountTimeoutError<'a, Chain>> for CosmosErrorHandlers
where
    Chain: HasAddressType + HasAmountType,
{
    type Delegate = DebugError;
}

impl<'a, Chain> DelegateComponent<BroadcastTxError<'a, Chain>> for CosmosErrorHandlers {
    type Delegate = DebugError;
}

impl<'a, Chain> DelegateComponent<TxNoResponseError<'a, Chain>> for CosmosErrorHandlers
where
    Chain: HasTransactionHashType,
{
    type Delegate = DebugError;
}

impl<'a, Chain, Counterparty>
    DelegateComponent<MissingCreateClientEventError<'a, Chain, Counterparty>>
    for CosmosErrorHandlers
where
    Chain: HasChainIdType,
    Counterparty: HasChainIdType,
{
    type Delegate = DebugError;
}

impl<'a, Context> DelegateComponent<MaxRetryExceededError<'a, Context>> for CosmosErrorHandlers
where
    Context: HasErrorType,
{
    type Delegate = UnwrapMaxRetryExceededError;
}
