use alloc::string::FromUtf8Error;
use core::convert::Infallible;
use core::num::ParseIntError;

use cgp_core::error::{DelegateErrorRaiser, ErrorRaiser, ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use eyre::Report;
use hermes_any_counterparty::impls::encoding::client_state::UnknownClientStateType;
use hermes_any_counterparty::impls::encoding::consensus_state::UnknownConsensusStateType;
use hermes_cosmos_chain_components::impls::queries::abci::AbciQueryError;
use hermes_cosmos_chain_components::impls::transaction::submit_tx::BroadcastTxError;
use hermes_error::handlers::debug::DebugError;
use hermes_error::handlers::display::DisplayError;
use hermes_error::handlers::identity::ReturnError;
use hermes_error::handlers::infallible::HandleInfallible;
use hermes_error::handlers::report::ReportError;
use hermes_error::handlers::wrap::WrapErrorDetail;
use hermes_error::impls::ProvideHermesError;
use hermes_error::traits::wrap::WrapError;
use hermes_error::types::Error;
use hermes_protobuf_encoding_components::impls::any::TypeUrlMismatchError;
use hermes_relayer_components::chain::impls::queries::consensus_state_height::NoConsensusStateAtLessThanHeight;
use hermes_relayer_components::chain::traits::queries::connection_end::ConnectionNotFoundError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
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
use ibc::core::client::types::error::ClientError;
use ibc::core::commitment_types::error::CommitmentError;
use ibc_relayer::error::Error as RelayerError;
use ibc_relayer::keyring::errors::Error as KeyringError;
use ibc_relayer::supervisor::Error as SupervisorError;
use ibc_relayer_types::clients::ics07_tendermint::error::Error as TendermintClientError;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics03_connection::error::Error as Ics03Error;
use ibc_relayer_types::core::ics23_commitment::error::Error as Ics23Error;
use ibc_relayer_types::core::ics24_host::error::ValidationError as Ics24ValidationError;
use ibc_relayer_types::proofs::ProofError;
use ibc_relayer_types::signer::SignerError;
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
    Context: HasErrorType<Error = Error>,
{
}

impl<Context> CanHandleCosmosError<Context> for HandleCosmosError where
    Context: HasErrorType<Error = Error>
{
}

delegate_components! {
    HandleCosmosError {
        [
            ErrorTypeComponent,
            RetryableErrorComponent,
        ]: ProvideHermesError,
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
            TendermintError,
            TendermintProtoError,
            TendermintRpcError,
            TendermintClientError,
            Ics02Error,
            Ics03Error,
            Ics23Error,
            Ics24ValidationError,
            ParseIntError,
            FromUtf8Error,
            EncodeError,
            DecodeError,
            InvalidMetadataValue,
            ProofError,
            ClientError,
            CommitmentError,

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
            UnknownClientStateType,
            UnknownConsensusStateType,
            AbciQueryError,
            Status,
            MissingSendPacketEventError,
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
            WrapError<&'static str, Error>,
            WrapError<String, Error>,
        ]:
            WrapErrorDetail,
        <'a, Context: HasErrorType> MaxRetryExceededError<'a, Context>:
            UnwrapMaxRetryExceededError,
    }
}
