use cgp_core::prelude::*;
use hermes_cosmos_client_components::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use hermes_cosmos_client_components::impls::packet::packet_fields::CosmosPacketFieldReader;
use hermes_cosmos_client_components::impls::queries::client_state::CosmosQueryClientStateComponents;
use hermes_cosmos_client_components::impls::types::chain::ProvideCosmosChainTypes;
use hermes_protobuf_components::types::Any;
use hermes_relayer_components::chain::impls::queries::client_state::QueryAndDecodeClientStateVia;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::ChannelHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::encode::impls::default_encoding::GetDefaultEncoding;
use hermes_relayer_components::encode::traits::has_encoding::EncodingGetterComponent;
use hermes_relayer_components::logger::traits::has_logger::{
    LoggerFieldComponent, LoggerTypeComponent,
};
use hermes_relayer_components::runtime::traits::runtime::RuntimeTypeComponent;
use hermes_relayer_runtime::impls::logger::components::ProvideTracingLogger;
use hermes_relayer_runtime::impls::types::runtime::ProvideHermesRuntime;

use crate::impls::chain::solomachine_components::channel_handshake_message::BuildCosmosToSolomachineChannelHandshakeMessage;
use crate::impls::chain::solomachine_components::channel_handshake_payload::BuildSolomachineChannelHandshakePayloads;
use crate::impls::chain::solomachine_components::connection_handshake_message::BuildCosmosToSolomachineConnectionHandshakeMessage;
use crate::impls::chain::solomachine_components::connection_handshake_payload::BuildSolomachineConnectionHandshakePayloads;
use crate::impls::chain::solomachine_components::create_client_message::BuildCreateCosmosClientMessage;
use crate::impls::chain::solomachine_components::create_client_payload::BuildSolomachineCreateClientPayload;
use crate::impls::chain::solomachine_components::process_message::ProcessSolomachineMessages;
use crate::impls::chain::solomachine_components::query_chain_status::QuerySolomachineStatus;
use crate::impls::chain::solomachine_components::query_client_state::QueryCosmosClientStateFromSolomachine;
use crate::impls::chain::solomachine_components::query_consensus_state::QueryCosmosConsensusStateFromSolomachine;
use crate::impls::chain::solomachine_components::receive_packet_payload::BuildSolomachineReceivePacketPayload;
use crate::impls::chain::solomachine_components::timeout_packet_payload::BuildSolomachineTimeoutPacketPayload;
use crate::impls::chain::solomachine_components::types::chain::ProvideSolomachineChainTypes;
use crate::impls::chain::solomachine_components::update_client_payload::BuildSolomachineUpdateClientPayload;
use crate::types::chain::SolomachineChain;

pub struct SolomachineChainComponents;

impl<Chain> HasComponents for SolomachineChain<Chain>
where
    Chain: Async,
{
    type Components = SolomachineChainComponents;
}

impl<Chain> DelegateComponent<SolomachineChain<Chain>> for CosmosQueryClientStateComponents {
    type Delegate = QueryAndDecodeClientStateVia<Any>;
}

delegate_components! {
    SolomachineChainComponents {
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            HeightTypeComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            LoggerTypeComponent,
            LoggerFieldComponent,
        ]:
            ProvideTracingLogger,
        [
            MessageTypeComponent,
            EventTypeComponent,
        ]:
            ProvideSolomachineChainTypes,
        EncodingGetterComponent:
            GetDefaultEncoding,
        PacketFieldsReaderComponent:
            CosmosPacketFieldReader,
        MessageSenderComponent:
            ProcessSolomachineMessages,
        ChainStatusQuerierComponent:
            QuerySolomachineStatus,
        ClientStateQuerierComponent:
            QueryCosmosClientStateFromSolomachine,
        ConsensusStateQuerierComponent:
            QueryCosmosConsensusStateFromSolomachine,
        ChannelHandshakePayloadBuilderComponent:
            BuildSolomachineChannelHandshakePayloads,
        ChannelHandshakeMessageBuilderComponent:
            BuildCosmosToSolomachineChannelHandshakeMessage,
        ConnectionHandshakePayloadBuilderComponent:
            BuildSolomachineConnectionHandshakePayloads,
        ConnectionHandshakeMessageBuilderComponent:
            BuildCosmosToSolomachineConnectionHandshakeMessage,
        CreateClientPayloadBuilderComponent:
            BuildSolomachineCreateClientPayload,
        CreateClientMessageBuilderComponent:
            BuildCreateCosmosClientMessage,
        ReceivePacketPayloadBuilderComponent:
            BuildSolomachineReceivePacketPayload,
        TimeoutUnorderedPacketMessageBuilderComponent:
            BuildSolomachineTimeoutPacketPayload,
        UpdateClientPayloadBuilderComponent:
            BuildSolomachineUpdateClientPayload,
        UpdateClientMessageBuilderComponent:
            BuildCosmosUpdateClientMessage,
    }
}
