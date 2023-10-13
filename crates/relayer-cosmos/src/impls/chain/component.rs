use cgp_core::prelude::*;
use ibc_cosmos_client_components::components::ack_packet_message::BuildCosmosAckPacketMessage;
use ibc_cosmos_client_components::components::ack_packet_payload::BuildCosmosAckPacketPayload;
use ibc_cosmos_client_components::components::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use ibc_cosmos_client_components::components::channel_handshake_payload::BuildCosmosChannelHandshakePayload;
use ibc_cosmos_client_components::components::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use ibc_cosmos_client_components::components::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use ibc_cosmos_client_components::components::packet_fields::CosmosPacketFieldReader;
use ibc_cosmos_client_components::components::packet_from_ack::BuildCosmosPacketFromWriteAck;
use ibc_cosmos_client_components::components::query_chain_id::QueryChainIdWithChainHandle;
use ibc_cosmos_client_components::components::query_chain_status::QueryChainStatusWithChainHandle;
use ibc_cosmos_client_components::components::query_consensus_state_height::QueryConsensusStateHeightFromChainHandle;
use ibc_cosmos_client_components::components::query_packet_commitments::QueryCosmosPacketCommitments;
use ibc_cosmos_client_components::components::query_received_packet::QueryReceivedPacketWithChainHandle;
use ibc_cosmos_client_components::components::query_send_packet::QueryCosmosSendPacket;
use ibc_cosmos_client_components::components::query_send_packets::QuerySendPacketsConcurrently;
use ibc_cosmos_client_components::components::query_unreceived_packet::QueryUnreceivedCosmosPacketSequences;
use ibc_cosmos_client_components::components::query_write_ack_event::QueryWriteAckEventFromChainHandle;
use ibc_cosmos_client_components::components::receive_packet_message::BuildCosmosReceivePacketMessage;
use ibc_cosmos_client_components::components::receive_packet_payload::BuildCosmosReceivePacketPayload;
use ibc_cosmos_client_components::components::send_messages_as_tx::SendMessagesToTxContext;
use ibc_cosmos_client_components::components::timeout_packet_message::BuildCosmosTimeoutPacketMessage;
use ibc_cosmos_client_components::components::timeout_packet_payload::BuildCosmosTimeoutPacketPayload;
use ibc_cosmos_client_components::components::types::chain::ProvideCosmosChainTypes;
use ibc_cosmos_client_components::components::update_client_message::BuildCosmosUpdateClientMessage;
use ibc_cosmos_client_components::components::update_client_payload::BuildUpdateClientPayloadWithChainHandle;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use ibc_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::consensus_state_height_querier::ConsensusStateHeightQuerierComponent;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use ibc_relayer_components::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use ibc_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerierComponent;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use ibc_relayer_components::chain::traits::components::packet_from_write_ack_builder::PacketFromWriteAckBuilderComponent;
use ibc_relayer_components::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
use ibc_relayer_components::chain::traits::components::send_packets_querier::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use ibc_relayer_components::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerierComponent;
use ibc_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
use ibc_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::height::HeightTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use ibc_relayer_components::chain::traits::types::status::ChainStatusTypeProviderComponent;
use ibc_relayer_components_extra::components::extra::chain::ExtraChainComponents;
use ibc_relayer_components_extra::components::extra::closures::chain::all::CanUseExtraChainComponents;

use crate::contexts::chain::CosmosChain;
use crate::impls::chain::components::connection_handshake_message::DelegateCosmosConnectionHandshakeBuilder;
use crate::impls::chain::components::create_client_message::DelegateCosmosCreateClientMessageBuilder;
use crate::impls::chain::components::query_client_state::DelegateCosmosClientStateQuerier;
use crate::impls::chain::components::query_consensus_state::DelegateCosmosConsensusStateQuerier;

pub struct CosmosChainComponents;

impl<Chain> HasComponents for CosmosChain<Chain>
where
    Chain: Async,
{
    type Components = ExtraChainComponents<CosmosChainComponents>;
}

impl<Chain, Counterparty> CanUseExtraChainComponents<CosmosChain<Counterparty>>
    for CosmosChain<Chain>
where
    Chain: ChainHandle,
    Counterparty: ChainHandle,
{
}

delegate_components!(
    CosmosChainComponents;
    [
        HeightTypeProviderComponent,
        MessageTypeProviderComponent,
        EventTypeProviderComponent,
        ChainStatusTypeProviderComponent,
    ]:
        ProvideCosmosChainTypes,
    MessageSenderComponent:
        SendMessagesToTxContext,
    ChainStatusQuerierComponent:
        QueryChainStatusWithChainHandle,
    PacketFieldsReaderComponent:
        CosmosPacketFieldReader,
    ClientStateQuerierComponent:
        DelegateCosmosClientStateQuerier,
    ConsensusStateQuerierComponent:
        DelegateCosmosConsensusStateQuerier,
    ConsensusStateHeightQuerierComponent:
        QueryConsensusStateHeightFromChainHandle,
    WriteAckQuerierComponent:
        QueryWriteAckEventFromChainHandle,
    CreateClientMessageBuilderComponent:
        DelegateCosmosCreateClientMessageBuilder,
    CreateClientPayloadBuilderComponent:
        BuildCreateClientPayloadWithChainHandle,
    UpdateClientPayloadBuilderComponent:
        BuildUpdateClientPayloadWithChainHandle,
    UpdateClientMessageBuilderComponent:
        BuildCosmosUpdateClientMessage,
    CounterpartyChainIdQuerierComponent:
        QueryChainIdWithChainHandle,
    ConnectionHandshakePayloadBuilderComponent:
        BuildCosmosConnectionHandshakePayload,
    ChannelHandshakePayloadBuilderComponent:
        BuildCosmosChannelHandshakePayload,
    ConnectionHandshakeMessageBuilderComponent:
        DelegateCosmosConnectionHandshakeBuilder,
    ChannelHandshakeMessageBuilderComponent:
        BuildCosmosChannelHandshakeMessage,
    PacketCommitmentsQuerierComponent:
        QueryCosmosPacketCommitments,
    ReceivedPacketQuerierComponent:
        QueryReceivedPacketWithChainHandle,
    ReceivePacketPayloadBuilderComponent:
        BuildCosmosReceivePacketPayload,
    ReceivePacketMessageBuilderComponent:
        BuildCosmosReceivePacketMessage,
    AckPacketPayloadBuilderComponent:
        BuildCosmosAckPacketPayload,
    AckPacketMessageBuilderComponent:
        BuildCosmosAckPacketMessage,
    TimeoutUnorderedPacketPayloadBuilderComponent:
        BuildCosmosTimeoutPacketPayload,
    TimeoutUnorderedPacketMessageBuilderComponent:
        BuildCosmosTimeoutPacketMessage,
    UnreceivedPacketSequencesQuerierComponent:
        QueryUnreceivedCosmosPacketSequences,
    SendPacketQuerierComponent:
        QueryCosmosSendPacket,
    SendPacketsQuerierComponent:
        QuerySendPacketsConcurrently,
    PacketFromWriteAckBuilderComponent:
        BuildCosmosPacketFromWriteAck,
);
