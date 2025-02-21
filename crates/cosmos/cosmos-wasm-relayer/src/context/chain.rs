use core::ops::Deref;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::traits::abci_query::CanQueryAbci;
use hermes_cosmos_chain_components::traits::gas_config::{
    GasConfigGetter, GasConfigGetterComponent,
};
use hermes_cosmos_chain_components::traits::grpc_address::{
    GrpcAddressGetter, GrpcAddressGetterComponent,
};
use hermes_cosmos_chain_components::traits::rpc_client::{
    RpcClientGetter, RpcClientGetterComponent,
};
use hermes_cosmos_chain_components::traits::tx_extension_options::{
    TxExtensionOptionsGetter, TxExtensionOptionsGetterComponent,
};
use hermes_cosmos_chain_components::traits::unbonding_period::CanQueryUnbondingPeriod;
use hermes_cosmos_chain_components::types::config::gas::gas_config::GasConfig;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_chain_components::types::nonce_guard::NonceGuard;
use hermes_cosmos_chain_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_chain_components::types::transaction::account::Account;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
    HasDefaultEncoding,
};
use hermes_encoding_components::types::AsBytes;
use hermes_logger::UseHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeProviderComponent,
};
use hermes_relayer_components::chain::impls::types::poll_interval::FixedPollIntervalMillis;
use hermes_relayer_components::chain::traits::commitment_prefix::{
    IbcCommitmentPrefixGetter, IbcCommitmentPrefixGetterComponent,
};
use hermes_relayer_components::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenInitMessage, CanBuildChannelOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::CanBuildReceivePacketMessage;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    CanBuildChannelOpenAckPayload, CanBuildChannelOpenConfirmPayload, CanBuildChannelOpenTryPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmPayload,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::CanBuildReceivePacketPayload;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketPayload;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    CanQueryChannelEnd, CanQueryChannelEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryAllClientStates, CanQueryClientState, CanQueryClientStateWithProofs,
    CanQueryRawClientState,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs, CanQueryRawConsensusState,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;
use hermes_relayer_components::chain::traits::queries::packet_commitment::CanQueryPacketCommitment;
use hermes_relayer_components::chain::traits::queries::packet_receipt::CanQueryPacketReceipt;
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ChainIdGetterComponent,
};
use hermes_relayer_components::chain::traits::types::channel::HasChannelEndType;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateType, HasRawClientStateType,
};
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::poll_interval::PollIntervalGetterComponent;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_relayer_components::error::traits::retry::{HasRetryableError, RetryableErrorComponent};
use hermes_relayer_components::transaction::impls::poll_tx_response::HasPollTimeout;
use hermes_relayer_components::transaction::traits::default_signer::{
    DefaultSignerGetter, DefaultSignerGetterComponent,
};
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::{
    MutexForNonceAllocationComponent, ProvideMutexForNonceAllocation,
};
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::simulation_fee::{
    FeeForSimulationGetter, FeeForSimulationGetterComponent,
};
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_wasm_test_components::components::WasmChainComponents;
use hermes_wasm_test_components::traits::chain::messages::store_code::StoreCodeMessageBuilderComponent;
use hermes_wasm_test_components::traits::chain::upload_client_code::{
    CanUploadWasmClientCode, WasmClientCodeUploaderComponent,
};
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::host::types::identifiers::ChainId;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use prost_types::Any;
use tendermint_rpc::{HttpClient, Url};

use crate::components::chain::{CosmosChainWasmPreset, IsCosmosChainWasmPreset};
use crate::components::cosmos_to_wasm_cosmos::CosmosToWasmCosmosComponents;
use crate::context::encoding::{ProvideWasmCosmosEncoding, WasmCosmosEncoding};
use crate::types::client_state::WasmTendermintClientState;

#[cgp_context(WasmCosmosChainComponents: CosmosChainWasmPreset)]
#[derive(Clone)]
pub struct WasmCosmosChain {
    pub chain: CosmosChain,
}

impl Deref for WasmCosmosChain {
    type Target = CosmosChain;

    fn deref(&self) -> &CosmosChain {
        &self.chain
    }
}

delegate_components! {
    WasmCosmosChainComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeProviderComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            UseHermesLogger,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
            DefaultEncodingGetterComponent,
        ]:
            ProvideWasmCosmosEncoding,
        [
            StoreCodeMessageBuilderComponent,
            WasmClientCodeUploaderComponent,
        ]:
            WasmChainComponents,

        PollIntervalGetterComponent:
            FixedPollIntervalMillis<200>,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        WasmCosmosChain: CosmosToWasmCosmosComponents,
    }
}

#[cgp_provider(TxExtensionOptionsGetterComponent)]
impl TxExtensionOptionsGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn tx_extension_options(chain: &WasmCosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.chain_config.extension_options
    }
}

#[cgp_provider(GasConfigGetterComponent)]
impl GasConfigGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn gas_config(chain: &WasmCosmosChain) -> &GasConfig {
        &chain.chain_config.gas_config
    }
}

#[cgp_provider(DefaultSignerGetterComponent)]
impl DefaultSignerGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn get_default_signer(chain: &WasmCosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

#[cgp_provider(FeeForSimulationGetterComponent)]
impl FeeForSimulationGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn fee_for_simulation(chain: &WasmCosmosChain) -> &Fee {
        &chain.chain_config.gas_config.max_fee
    }
}

#[cgp_provider(MutexForNonceAllocationComponent)]
impl ProvideMutexForNonceAllocation<WasmCosmosChain> for WasmCosmosChainComponents {
    fn mutex_for_nonce_allocation<'a>(
        chain: &'a WasmCosmosChain,
        _signer: &Secp256k1KeyPair,
    ) -> &'a Mutex<()> {
        &chain.nonce_mutex
    }

    fn mutex_to_nonce_guard(
        mutex_guard: MutexGuardOf<'_, HermesRuntime, ()>,
        account: Account,
    ) -> NonceGuard<'_> {
        NonceGuard {
            mutex_guard,
            account,
        }
    }
}

#[cgp_provider(IbcCommitmentPrefixGetterComponent)]
impl IbcCommitmentPrefixGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn ibc_commitment_prefix(chain: &WasmCosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}

#[cgp_provider(GrpcAddressGetterComponent)]
impl GrpcAddressGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn grpc_address(chain: &WasmCosmosChain) -> &Url {
        &chain.chain_config.grpc_addr
    }
}

#[cgp_provider(RpcClientGetterComponent)]
impl RpcClientGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn rpc_client(chain: &WasmCosmosChain) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &WasmCosmosChain) -> &Url {
        &chain.chain_config.rpc_addr
    }
}

impl HasTelemetry for WasmCosmosChain {
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}

#[cgp_provider(ChainIdGetterComponent)]
impl ChainIdGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn chain_id(chain: &WasmCosmosChain) -> &ChainId {
        &chain.chain_id
    }
}

pub trait CanUseWasmCosmosChain:
    HasClientStateType<WasmCosmosChain, ClientState = WasmTendermintClientState>
    + HasConsensusStateType<WasmCosmosChain, ConsensusState = TendermintConsensusState>
    + HasCreateClientPayloadType<WasmCosmosChain, CreateClientPayload = CosmosCreateClientPayload>
    + HasUpdateClientPayloadType<WasmCosmosChain, UpdateClientPayload = CosmosUpdateClientPayload>
    + CanQueryBalance
    // + CanIbcTransferToken<WasmCosmosChain>
    // + CanBuildIbcTokenTransferMessage<WasmCosmosChain>
    + CanQueryRawClientState<WasmCosmosChain>
    + CanQueryClientState<WasmCosmosChain>
    + CanQueryClientStateWithProofs<WasmCosmosChain>
    + CanQueryConsensusState<WasmCosmosChain>
    + CanQueryConsensusStateWithProofs<WasmCosmosChain>
    + CanQueryRawConsensusState<WasmCosmosChain>
    + CanQueryAllClientStates<WasmCosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryAllClientStates<AnyCounterparty>
    + CanBuildUpdateClientMessage<WasmCosmosChain>
    + CanQueryConnectionEnd<WasmCosmosChain>
    + CanQueryChannelEnd<WasmCosmosChain>
    + CanQueryChannelEndWithProofs<WasmCosmosChain>
    + CanQueryConnectionEndWithProofs<WasmCosmosChain>
    + CanQueryPacketCommitment<WasmCosmosChain>
    + CanQueryPacketAcknowledgement<WasmCosmosChain>
    + CanQueryPacketReceipt<WasmCosmosChain>
    + HasChannelEndType<WasmCosmosChain, ChannelEnd = ChannelEnd>
    + HasRawClientStateType<RawClientState = Any>
    + CanSubmitTx
    + CanPollTxResponse
    + HasPollTimeout
    + CanQueryTxResponse
    + HasRetryableError
    + HasLogger
    + HasRuntime
    // + CanAssertEventualAmount
    + CanQueryAbci
    + CanQueryUnbondingPeriod
    + CanQueryClientState<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanBuildConnectionOpenInitPayload<CosmosChain>
    + CanBuildConnectionOpenTryPayload<CosmosChain>
    + CanBuildConnectionOpenAckPayload<CosmosChain>
    + CanBuildConnectionOpenConfirmPayload<CosmosChain>
    + CanBuildConnectionOpenInitMessage<CosmosChain>
    + CanBuildConnectionOpenTryMessage<CosmosChain>
    + CanBuildConnectionOpenAckMessage<CosmosChain>
    + CanBuildConnectionOpenConfirmMessage<CosmosChain>
    + CanBuildChannelOpenTryPayload<CosmosChain>
    + CanBuildChannelOpenAckPayload<CosmosChain>
    + CanBuildChannelOpenConfirmPayload<CosmosChain>
    + CanBuildChannelOpenInitMessage<CosmosChain>
    + CanBuildChannelOpenTryMessage<CosmosChain>
    + CanBuildChannelOpenAckMessage<CosmosChain>
    + CanBuildChannelOpenConfirmMessage<CosmosChain>
    + CanBuildReceivePacketPayload<CosmosChain>
    + CanBuildAckPacketPayload<CosmosChain>
    + CanBuildTimeoutUnorderedPacketPayload<CosmosChain>
    + CanBuildReceivePacketMessage<CosmosChain>
    + CanBuildAckPacketMessage<CosmosChain>
    + CanBuildTimeoutUnorderedPacketMessage<CosmosChain>
    + HasInitConnectionOptionsType<CosmosChain>
    + HasCreateClientMessageOptionsType<
            CosmosChain,
            CreateClientMessageOptions = (),
        >
    + HasDefaultEncoding<AsBytes, Encoding = WasmCosmosEncoding>
    + CanUploadWasmClientCode
where
    CosmosChain: HasClientStateType<Self, ClientState = TendermintClientState>
        + HasConsensusStateType<Self, ConsensusState = TendermintConsensusState>
        + HasUpdateClientPayloadType<Self>
        + HasCreateClientPayloadType<Self>
        ,
    WasmCosmosChain: HasConsensusStateType<Self>
        + HasUpdateClientPayloadType<Self>
        + HasCreateClientPayloadType<Self>
{
}

pub trait CanUseCosmosChainWithWasmCosmosChain:
    CanQueryClientState<WasmCosmosChain>
    + CanQueryConsensusState<WasmCosmosChain>
    + CanBuildConnectionOpenInitPayload<WasmCosmosChain>
    + CanBuildConnectionOpenTryPayload<WasmCosmosChain>
    + CanBuildConnectionOpenAckPayload<WasmCosmosChain>
    + CanBuildConnectionOpenConfirmPayload<WasmCosmosChain>
    + CanBuildConnectionOpenInitMessage<WasmCosmosChain>
    + CanBuildConnectionOpenTryMessage<WasmCosmosChain>
    + CanBuildChannelOpenTryPayload<WasmCosmosChain>
    + CanBuildChannelOpenAckPayload<WasmCosmosChain>
    + CanBuildChannelOpenConfirmPayload<WasmCosmosChain>
    + CanBuildChannelOpenInitMessage<WasmCosmosChain>
    + CanBuildChannelOpenTryMessage<WasmCosmosChain>
    + CanBuildChannelOpenAckMessage<WasmCosmosChain>
    + CanBuildChannelOpenConfirmMessage<WasmCosmosChain>
    + CanBuildReceivePacketPayload<WasmCosmosChain>
    + CanBuildAckPacketPayload<WasmCosmosChain>
    + CanBuildTimeoutUnorderedPacketPayload<WasmCosmosChain>
    + CanBuildReceivePacketMessage<WasmCosmosChain>
    + CanBuildAckPacketMessage<WasmCosmosChain>
    + CanBuildTimeoutUnorderedPacketMessage<WasmCosmosChain>
    + HasInitConnectionOptionsType<WasmCosmosChain>
    + CanBuildCreateClientMessage<WasmCosmosChain>
where
    WasmCosmosChain: HasConsensusStateType<Self>
        + HasCreateClientPayloadType<Self>
        + HasUpdateClientPayloadType<Self>,
{
}

impl CanUseCosmosChainWithWasmCosmosChain for CosmosChain {}
