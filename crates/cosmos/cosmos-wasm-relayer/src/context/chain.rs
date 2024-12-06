use core::ops::Deref;
use std::sync::Arc;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::impls::use_field::WithField;
use cgp::core::types::impls::WithType;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_cosmos_chain_components::components::client::{
    ChannelIdTypeComponent, ClientIdTypeComponent, ClientStateFieldsComponent,
    ClientStateTypeComponent, ConnectionIdTypeComponent, CosmosClientComponents,
    MessageResponseEventsGetterComponent, MessageResponseTypeComponent,
    OutgoingPacketFieldsReaderComponent, OutgoingPacketTypeComponent, PortIdTypeComponent,
    SequenceTypeComponent, TimeTypeComponent, UnbondingPeriodQuerierComponent,
};
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::components::transaction::*;
use hermes_cosmos_chain_components::traits::abci_query::{AbciQuerierComponent, CanQueryAbci};
use hermes_cosmos_chain_components::traits::eip::eip_query::EipQuerierComponent;
use hermes_cosmos_chain_components::traits::gas_config::GasConfigGetter;
use hermes_cosmos_chain_components::traits::grpc_address::GrpcAddressGetter;
use hermes_cosmos_chain_components::traits::rpc_client::RpcClientGetter;
use hermes_cosmos_chain_components::traits::tx_extension_options::TxExtensionOptionsGetter;
use hermes_cosmos_chain_components::traits::unbonding_period::CanQueryUnbondingPeriod;
use hermes_cosmos_chain_components::types::config::gas::gas_config::GasConfig;
use hermes_cosmos_chain_components::types::nonce_guard::NonceGuard;
use hermes_cosmos_chain_components::types::payloads::client::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::impls::error::HandleCosmosError;
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use hermes_cosmos_test_components::chain::components::*;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
    HasDefaultEncoding,
};
use hermes_encoding_components::types::AsBytes;
use hermes_logger::ProvideHermesLogger;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, HasLogger, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::{
    CommitmentPrefixTypeComponent, IbcCommitmentPrefixGetter,
};
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::{
    AckPacketMessageBuilderComponent, CanBuildAckPacketMessage,
};
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenInitMessage, CanBuildChannelOpenTryMessage,
    ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::{
    CanBuildCreateClientMessage, CreateClientMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::receive_packet::{
    CanBuildReceivePacketMessage, ReceivePacketMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketMessage, TimeoutUnorderedPacketMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::update_client::{
    CanBuildUpdateClientMessage, UpdateClientMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::{
    AckPacketPayloadBuilderComponent, CanBuildAckPacketPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    CanBuildChannelOpenAckPayload, CanBuildChannelOpenConfirmPayload,
    CanBuildChannelOpenTryPayload, ChannelOpenAckPayloadBuilderComponent,
    ChannelOpenConfirmPayloadBuilderComponent, ChannelOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmPayload,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryPayload,
    ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::{
    CanBuildReceivePacketPayload, ReceivePacketPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketPayload, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::ack_packets::{
    AckPacketQuerierComponent, AckPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    CanQueryChannelEnd, CanQueryChannelEndWithProofs, ChannelEndQuerierComponent,
    ChannelEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, AllRawClientStatesQuerierComponent, CanQueryAllClientStates,
    CanQueryClientState, CanQueryClientStateWithProofs, CanQueryRawClientState,
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
    RawClientStateQuerierComponent, RawClientStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs, ConnectionEndQuerierComponent,
    ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs, CanQueryRawConsensusState,
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
    RawConsensusStateQuerierComponent, RawConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    ConsensusStateHeightQuerierComponent, ConsensusStateHeightsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::{
    CanQueryPacketAcknowledgement, PacketAcknowledgementQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_commitment::{
    CanQueryPacketCommitment, PacketCommitmentQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_receipt::{
    CanQueryPacketReceipt, PacketReceiptQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::send_packets::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::unreceived_acks_sequences::UnreceivedAcksSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerierComponent;
use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
use hermes_relayer_components::chain::traits::types::block::{
    BlockHashComponent, BlockTypeComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ChainIdTypeComponent,
};
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelEndTypeComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent, HasChannelEndType,
    InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateType, HasRawClientStateType, RawClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ConnectionOpenAckPayloadTypeComponent,
    ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitPayloadTypeComponent,
    ConnectionOpenTryPayloadTypeComponent, HasInitConnectionOptionsType,
    InitConnectionOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateTypeComponent, HasConsensusStateType, RawConsensusStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientMessageOptionsTypeComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetterComponent, HeightFieldComponent, HeightIncrementerComponent,
    HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetterComponent;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    ChannelOpenInitEventComponent, ChannelOpenTryEventComponent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ConnectionOpenInitEventComponent, ConnectionOpenTryEventComponent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::SendPacketEventComponent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::WriteAckEventComponent;
use hermes_relayer_components::chain::traits::types::message::{
    MessageSizeEstimatorComponent, MessageTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packets::ack::{
    AckPacketPayloadTypeComponent, AcknowledgementTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packets::receive::{
    PacketCommitmentTypeComponent, ReceivePacketPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packets::timeout::{
    PacketReceiptTypeComponent, TimeoutUnorderedPacketPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeComponent,
};
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimeoutTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::{
    HasUpdateClientPayloadType, UpdateClientPayloadTypeComponent,
};
use hermes_relayer_components::error::traits::retry::{HasRetryableError, RetryableErrorComponent};
use hermes_relayer_components::transaction::impls::poll_tx_response::HasPollTimeout;
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetter;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::ProvideMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_wasm_test_components::components::WasmChainComponents;
use hermes_wasm_test_components::traits::chain::messages::store_code::StoreCodeMessageBuilderComponent;
use hermes_wasm_test_components::traits::chain::upload_client_code::{
    CanUploadWasmClientCode, WasmClientCodeUploaderComponent,
};
use ibc::core::channel::types::channel::ChannelEnd;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;
use prost_types::Any;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::{HttpClient, Url};

use crate::components::cosmos_to_wasm_cosmos::CosmosToWasmCosmosComponents;
use crate::context::encoding::{ProvideWasmCosmosEncoding, WasmCosmosEncoding};
use crate::impls::client_state::ProvideWrappedTendermintClientState;
use crate::types::client_state::WasmTendermintClientState;

#[derive(Clone)]
pub struct WasmCosmosChain {
    pub chain: CosmosChain,
}

pub struct WasmCosmosChainComponents;

impl Deref for WasmCosmosChain {
    type Target = CosmosChain;

    fn deref(&self) -> &CosmosChain {
        &self.chain
    }
}

impl HasComponents for WasmCosmosChain {
    type Components = WasmCosmosChainComponents;
}

delegate_components! {
    WasmCosmosChainComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
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
    }
}

delegate_components! {
    WasmCosmosChainComponents {
        [
            CreateClientMessageOptionsTypeComponent,
            CreateClientMessageBuilderComponent,
            CreateClientPayloadTypeComponent,
            CreateClientPayloadOptionsTypeComponent,
            CreateClientPayloadBuilderComponent,
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            GenesisHeightGetterComponent,
            TimeTypeComponent,
            TimeoutTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            MessageResponseTypeComponent,
            MessageResponseEventsGetterComponent,
            MessageSizeEstimatorComponent,
            EventTypeComponent,
            RawClientStateTypeComponent,
            RawConsensusStateTypeComponent,
            ConsensusStateTypeComponent,
            ClientIdTypeComponent,
            ConnectionIdTypeComponent,
            ChannelIdTypeComponent,
            PortIdTypeComponent,
            SequenceTypeComponent,

            ConnectionEndQuerierComponent,
            ConnectionEndWithProofsQuerierComponent,
            ConnectionEndTypeComponent,

            ChannelEndQuerierComponent,
            ChannelEndWithProofsQuerierComponent,
            ChannelEndTypeComponent,

            OutgoingPacketTypeComponent,
            ChainStatusTypeComponent,
            BlockTypeComponent,
            BlockHashComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            CommitmentProofHeightGetterComponent,
            CommitmentProofBytesGetterComponent,
            PacketCommitmentTypeComponent,
            AcknowledgementTypeComponent,
            PacketReceiptTypeComponent,

            CreateClientEventComponent,
            ConnectionOpenInitEventComponent,
            ConnectionOpenTryEventComponent,
            ChannelOpenInitEventComponent,
            ChannelOpenTryEventComponent,
            SendPacketEventComponent,
            WriteAckEventComponent,

            UpdateClientPayloadTypeComponent,

            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,

            ChannelOpenTryPayloadTypeComponent,
            ChannelOpenAckPayloadTypeComponent,
            ChannelOpenConfirmPayloadTypeComponent,

            ReceivePacketPayloadTypeComponent,
            AckPacketPayloadTypeComponent,
            TimeoutUnorderedPacketPayloadTypeComponent,

            OutgoingPacketFieldsReaderComponent,
            WriteAckQuerierComponent,

            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            RawClientStateQuerierComponent,
            RawClientStateWithProofsQuerierComponent,
            AllClientStatesQuerierComponent,
            AllRawClientStatesQuerierComponent,

            RawConsensusStateQuerierComponent,
            RawConsensusStateWithProofsQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,

            ConsensusStateHeightQuerierComponent,
            ConsensusStateHeightsQuerierComponent,

            UpdateClientMessageBuilderComponent,
            UpdateClientPayloadBuilderComponent,
            CounterpartyChainIdQuerierComponent,

            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,
            ConnectionOpenInitPayloadBuilderComponent,
            ConnectionOpenTryPayloadBuilderComponent,
            ConnectionOpenAckPayloadBuilderComponent,
            ConnectionOpenConfirmPayloadBuilderComponent,

            ChannelOpenTryPayloadBuilderComponent,
            ChannelOpenAckPayloadBuilderComponent,
            ChannelOpenConfirmPayloadBuilderComponent,

            ChannelOpenInitMessageBuilderComponent,
            ChannelOpenTryMessageBuilderComponent,
            ChannelOpenAckMessageBuilderComponent,
            ChannelOpenConfirmMessageBuilderComponent,

            PacketCommitmentQuerierComponent,
            PacketCommitmentsQuerierComponent,

            PacketAcknowledgementQuerierComponent,
            PacketAcknowledgementsQuerierComponent,

            PacketReceiptQuerierComponent,

            ReceivedPacketQuerierComponent,

            ReceivePacketPayloadBuilderComponent,
            ReceivePacketMessageBuilderComponent,

            AckPacketPayloadBuilderComponent,
            AckPacketMessageBuilderComponent,

            TimeoutUnorderedPacketPayloadBuilderComponent,
            TimeoutUnorderedPacketMessageBuilderComponent,

            UnreceivedPacketSequencesQuerierComponent,
            UnreceivedAcksSequencesQuerierComponent,

            AckPacketQuerierComponent,
            AckPacketsQuerierComponent,
            SendPacketQuerierComponent,
            SendPacketsQuerierComponent,
            PacketFromWriteAckBuilderComponent,
            InitConnectionOptionsTypeComponent,
            InitChannelOptionsTypeComponent,
            BlockQuerierComponent,
            AbciQuerierComponent,
            UnbondingPeriodQuerierComponent,
            CounterpartyMessageHeightGetterComponent,
            ChainStatusQuerierComponent,
            ConsensusStateQuerierComponent,
        ]:
            CosmosClientComponents,
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,
        ]:
            ProvideWrappedTendermintClientState,
    }
}

with_cosmos_tx_components! {
    delegate_components! {
        WasmCosmosChainComponents {
            @CosmosTxComponents : CosmosTxComponents,
        }
    }
}

with_cosmmos_chain_test_components! {
    delegate_components! {
        WasmCosmosChainComponents {
            @CosmmosChainTestComponents: CosmmosChainTestComponents,
        }
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        WasmCosmosChain: CosmosToWasmCosmosComponents,
    }
}

impl TxExtensionOptionsGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn tx_extension_options(chain: &WasmCosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.chain_config.extension_options
    }
}

impl GasConfigGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn gas_config(chain: &WasmCosmosChain) -> &GasConfig {
        &chain.chain_config.gas_config
    }
}

impl DefaultSignerGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn get_default_signer(chain: &WasmCosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

impl FeeForSimulationGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn fee_for_simulation(chain: &WasmCosmosChain) -> &Fee {
        &chain.chain_config.gas_config.max_fee
    }
}

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

impl IbcCommitmentPrefixGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn ibc_commitment_prefix(chain: &WasmCosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}

impl GrpcAddressGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn grpc_address(chain: &WasmCosmosChain) -> &Url {
        &chain.chain_config.grpc_addr
    }
}

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

impl ChainIdGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn chain_id(chain: &WasmCosmosChain) -> &ChainId {
        &chain.chain_id
    }
}

impl HasEventSubscription for WasmCosmosChain {
    fn event_subscription(&self) -> &Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>> {
        &self.subscription
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

impl CanUseWasmCosmosChain for WasmCosmosChain {}

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
