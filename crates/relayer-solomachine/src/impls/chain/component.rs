use cgp_core::prelude::*;
use ibc_cosmos_client_components::components::packet_fields::CosmosPacketFieldReader;
use ibc_cosmos_client_components::components::types::chain::ProvideCosmosChainTypes;
use ibc_cosmos_client_components::components::update_client_message::BuildCosmosUpdateClientMessage;
use ibc_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use ibc_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::TimeoutUnorderedPacketMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::height::HeightTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use ibc_relayer_components::components::default::chain::DefaultChainComponents;

use crate::impls::chain::solomachine_components::channel_handshake_message::BuildCosmosToSolomachineChannelHandshakeMessage;
use crate::impls::chain::solomachine_components::channel_handshake_payload::BuildSolomachineChannelHandshakePayloads;
use crate::impls::chain::solomachine_components::connection_handshake_message::BuildCosmosToSolomachineConnectionHandshakeMessage;
use crate::impls::chain::solomachine_components::connection_handshake_payload::BuildSolomachineConnectionHandshakePayloads;
use crate::impls::chain::solomachine_components::create_client_message::BuildCreateCosmosClientMessage;
use crate::impls::chain::solomachine_components::create_client_payload::BuildSolomachineCreateClientPayload;
use crate::impls::chain::solomachine_components::process_message::ProcessSolomachineMessages;
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
    type Components = DefaultChainComponents<SolomachineChainComponents>;
}

delegate_components!(
    SolomachineChainComponents;
    [
        HeightTypeProviderComponent,
        ChainIdTypeProviderComponent,
    ]:
        ProvideCosmosChainTypes,
    [
        MessageTypeProviderComponent,
        EventTypeProviderComponent,
    ]:
        ProvideSolomachineChainTypes,
    PacketFieldsReaderComponent:
        CosmosPacketFieldReader,
    MessageSenderComponent:
        ProcessSolomachineMessages,
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
);
