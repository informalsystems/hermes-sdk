use alloc::string::FromUtf8Error;
use core::array::TryFromSliceError;
use core::convert::Infallible;
use core::num::{ParseFloatError, ParseIntError, TryFromIntError};
use core::str::Utf8Error;

use cgp::core::component::UseDelegate;
use cgp::core::error::{
    ErrorRaiser, ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent,
};
use cgp::prelude::*;
use eyre::Report;
use hermes_any_counterparty::impls::encoding::client_state::UnknownClientStateType;
use hermes_any_counterparty::impls::encoding::consensus_state::UnknownConsensusStateType;
use hermes_cosmos_chain_components::impls::queries::abci::AbciQueryError;
use hermes_cosmos_chain_components::impls::queries::eip::types::EipQueryError;
use hermes_cosmos_chain_components::impls::transaction::submit_tx::BroadcastTxError;
use hermes_cosmos_test_components::chain::impls::proposal::query_status::ProposalFailed;
use hermes_error::handlers::debug::DebugError;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_protobuf_encoding_components::impls::any::TypeUrlMismatchError;
use hermes_protobuf_encoding_components::impls::encode_mut::chunk::{
    InvalidWireType, UnsupportedWireType,
};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::RequiredFieldTagNotFound;
use hermes_relayer_components::chain::impls::payload_builders::packet::InvalidTimeoutReceipt;
use hermes_relayer_components::chain::impls::queries::consensus_state_height::NoConsensusStateAtLessThanHeight;
use hermes_relayer_components::chain::traits::queries::connection_end::ConnectionNotFoundError;
use hermes_relayer_components::chain::traits::send_message::EmptyMessageResponse;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::error::impls::error::{
    MaxRetryExceededError, UnwrapMaxRetryExceededError,
};
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::relay::impls::channel::open_init::MissingChannelInitEventError;
use hermes_relayer_components::relay::impls::channel::open_try::MissingChannelTryEventError;
use hermes_relayer_components::relay::impls::connection::open_init::MissingConnectionInitEventError;
use hermes_relayer_components::relay::impls::connection::open_try::MissingConnectionTryEventError;
use hermes_relayer_components::relay::impls::create_client::MissingCreateClientEventError;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::types::tx_hash::HasTransactionHashType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_test_components::chain::impls::assert::poll_assert_eventual_amount::EventualAmountTimeoutError;
use hermes_test_components::chain::impls::ibc_transfer::MissingSendPacketEventError;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_wasm_test_components::impls::chain::upload_client_code::ProposalIdNotFound;
use http::uri::InvalidUri;
use ibc::clients::tendermint::types::error::TendermintClientError;
use ibc::core::channel::types::error::ChannelError;
use ibc::core::client::types::error::ClientError;
use ibc::core::commitment_types::error::CommitmentError;
use ibc::core::host::types::error::{DecodingError, IdentifierError};
use ibc::primitives::TimestampError;
use prost::{DecodeError, EncodeError};
use tendermint::Error as TendermintError;
use tendermint_proto::Error as TendermintProtoError;
use tendermint_rpc::Error as TendermintRpcError;
use tonic::metadata::errors::InvalidMetadataValue;
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::contexts::chain::CosmosChain;

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
    Context: HasAsyncErrorType<Error = Error>,
{
}

impl<Context> CanHandleCosmosError<Context> for HandleCosmosError where
    Context: HasAsyncErrorType<Error = Error>
{
}

delegate_components! {
    HandleCosmosError {
        [
            ErrorTypeProviderComponent,
            RetryableErrorComponent,
        ]: UseHermesError,
        ErrorWrapperComponent:
            WrapErrorDetail,
        ErrorRaiserComponent:
            UseDelegate<CosmosErrorHandlers>,
    }
}

delegate_components! {
    CosmosErrorHandlers {
        Error: ReturnError,
        Infallible: HandleInfallible,
        [
            Report,
            TokioRuntimeError,
            TendermintError,
            TendermintClientError,
            TendermintProtoError,
            TendermintRpcError,
            TimestampError,
            ChannelError,
            DecodingError,
            IdentifierError,
            ParseIntError,
            ParseFloatError,
            FromUtf8Error,
            EncodeError,
            DecodeError,
            InvalidMetadataValue,
            ClientError,
            CommitmentError,
            Utf8Error,
            TryFromIntError,
            TryFromSliceError,
            subtle_encoding::Error,
            reqwest::Error,
            InvalidUri,

            // TODO: make it retryable?
            TransportError,
        ]: ReportError,
        [
            <'a> &'a str,
            String,
        ]:
            DisplayError,
        [
            TypeUrlMismatchError,
            UnsupportedWireType,
            InvalidWireType,
            RequiredFieldTagNotFound,
            UnknownClientStateType,
            UnknownConsensusStateType,
            AbciQueryError,
            EipQueryError,
            Status,
            MissingSendPacketEventError,
            ProposalIdNotFound,
            EmptyMessageResponse,
            <'a, Chain: HasAddressType + HasAmountType>
                EventualAmountTimeoutError<'a, Chain>,
            <'a, Chain>
                BroadcastTxError<'a, Chain>,
            <'a, Chain: HasTransactionHashType>
                TxNoResponseError<'a, Chain>,
            <'a, Chain: HasIbcChainTypes<Counterparty>, Counterparty: HasHeightType>
                NoConsensusStateAtLessThanHeight<'a, Chain, Counterparty>,
            <'a, Chain: HasChainIdType, Counterparty: HasChainIdType>
                MissingCreateClientEventError<'a, Chain, Counterparty>,
            <'a, Chain: HasIbcChainTypes<Counterparty>, Counterparty>
                ConnectionNotFoundError<'a, Chain, Counterparty>,
            <'a, Chain: HasHeightType, Counterparty: HasOutgoingPacketType<Chain>>
                InvalidTimeoutReceipt<'a, Chain, Counterparty>,
            <'a, Chain>
                ProposalFailed<'a, Chain>,
            <'a, Relay>
                MissingConnectionInitEventError<'a, Relay>,
            <'a, Relay: HasRelayChains>
                MissingConnectionTryEventError<'a, Relay>,
            <'a, Relay>
                MissingChannelInitEventError<'a, Relay>,
            <'a, Relay: HasRelayChains>
                MissingChannelTryEventError<'a, Relay>,
        ]:
            DebugError,
        <'a, Context: HasAsyncErrorType> MaxRetryExceededError<'a, Context>:
            UnwrapMaxRetryExceededError,
    }
}
