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
use futures::channel::mpsc::SendError;
use futures::channel::oneshot::Canceled;
use hermes_any_counterparty::impls::encoding::client_state::UnknownClientStateType;
use hermes_any_counterparty::impls::encoding::consensus_state::UnknownConsensusStateType;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_cosmos_chain_components::impls::queries::abci::AbciQueryError;
use hermes_cosmos_chain_components::impls::queries::eip::types::EipQueryError;
use hermes_cosmos_chain_components::impls::transaction::submit_tx::BroadcastTxError;
use hermes_cosmos_test_components::chain::impls::proposal::query_status::ProposalFailed;
use hermes_error::handlers::debug::{DebugError, DebugRetryableError};
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::{ReportError, ReportRetryableError};
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::impls::UseHermesError;
use hermes_error::types::Error;
use hermes_protobuf_encoding_components::impls::any::TypeUrlMismatchError;
use hermes_protobuf_encoding_components::impls::encode_mut::chunk::{
    InvalidWireType, UnsupportedWireType,
};
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::RequiredFieldTagNotFound;
use hermes_relayer_components::chain::impls::{
    InvalidTimeoutReceipt, NoConsensusStateAtLessThanHeight,
};
use hermes_relayer_components::chain::traits::{
    ConnectionNotFoundError, EmptyMessageResponse, HasChainIdType, HasHeightType, HasIbcChainTypes,
    HasOutgoingPacketType,
};
use hermes_relayer_components::error::traits::RetryableErrorComponent;
use hermes_relayer_components::relay::impls::{
    MissingChannelInitEventError, MissingChannelTryEventError, MissingConnectionInitEventError,
    MissingConnectionTryEventError, MissingCreateClientEventError,
};
use hermes_relayer_components::relay::traits::HasRelayChains;
use hermes_relayer_components::transaction::impls::TxNoResponseError;
use hermes_relayer_components::transaction::traits::HasTxHashType;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_test_components::chain::impls::{
    EventualAmountTimeoutError, MissingSendPacketEventError,
};
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
            SendError,
            Canceled,

            // TODO: make it retryable?
            TransportError,
        ]: ReportError,
        [
            TendermintRpcError,
        ]:
            ReportRetryableError,
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
            EipQueryError,
            Status,
            MissingSendPacketEventError,
            ProposalIdNotFound,
            EmptyMessageResponse,
            <'a, Chain: HasAddressType + HasAmountType>
                EventualAmountTimeoutError<'a, Chain>,
            <'a, Chain>
                BroadcastTxError<'a, Chain>,
            <'a, Chain: HasTxHashType>
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
        [
            AbciQueryError,
        ]:
            DebugRetryableError,
    }
}
