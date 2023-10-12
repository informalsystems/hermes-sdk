use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use ibc_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::TimeoutUnorderedPacketMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use ibc_relayer_components::components::default::chain::DefaultChainComponents;
use ibc_relayer_cosmos::impls::chain::components::packet_fields::CosmosPacketFieldReader;

use crate::impls::chain::components::channel_handshake_payload::BuildSolomachineChannelHandshakePayloads;
use crate::impls::chain::components::connection_handshake_payload::BuildSolomachineConnectionHandshakePayloads;
use crate::impls::chain::components::create_client_payload::BuildSolomachineCreateClientPayload;
use crate::impls::chain::components::process_message::ProcessSolomachineMessages;
use crate::impls::chain::components::query_client_state::QueryCosmosClientStateFromSolomachine;
use crate::impls::chain::components::query_consensus_state::QueryCosmosConsensusStateFromSolomachine;
use crate::impls::chain::components::receive_packet_payload::BuildSolomachineReceivePacketPayload;
use crate::impls::chain::components::timeout_packet_payload::BuildSolomachineTimeoutPacketPayload;
use crate::impls::chain::components::update_client_payload::BuildSolomachineUpdateClientPayload;
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
    ConnectionHandshakePayloadBuilderComponent:
        BuildSolomachineConnectionHandshakePayloads,
    CreateClientPayloadBuilderComponent:
        BuildSolomachineCreateClientPayload,
    ReceivePacketPayloadBuilderComponent:
        BuildSolomachineReceivePacketPayload,
    TimeoutUnorderedPacketMessageBuilderComponent:
        BuildSolomachineTimeoutPacketPayload,
    UpdateClientPayloadBuilderComponent:
        BuildSolomachineUpdateClientPayload,
);
