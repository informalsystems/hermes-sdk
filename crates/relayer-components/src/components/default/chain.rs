use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilderComponent;
use crate::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilderComponent;
use crate::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use crate::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilderComponent;
use crate::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilderComponent;
use crate::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use crate::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilderComponent;
use crate::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use crate::chain::traits::components::consensus_state_height_querier::ConsensusStateHeightQuerierComponent;
use crate::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use crate::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use crate::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;
use crate::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use crate::chain::traits::components::message_sender::MessageSenderComponent;
use crate::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerierComponent;
use crate::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use crate::chain::traits::components::packet_from_write_ack_builder::PacketFromWriteAckBuilderComponent;
use crate::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilderComponent;
use crate::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use crate::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
use crate::chain::traits::components::send_packets_querier::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
use crate::chain::traits::components::timeout_unordered_packet_message_builder::{
    TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use crate::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerierComponent;
use crate::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use crate::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use crate::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
use crate::chain::traits::types::height::HeightTypeProviderComponent;
use crate::chain::traits::types::message::MessageTypeProviderComponent;
pub struct DefaultChainComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_components!(
    DefaultChainComponents<BaseComponents>;
    [
        HeightTypeProviderComponent,
        MessageTypeProviderComponent,
        ChainStatusQuerierComponent,
        ConsensusStateQuerierComponent,
        MessageSenderComponent,
        PacketFieldsReaderComponent,
        CounterpartyChainIdQuerierComponent,
        PacketCommitmentsQuerierComponent,
        ReceivedPacketQuerierComponent,
        SendPacketQuerierComponent,
        SendPacketsQuerierComponent,
        UnreceivedPacketSequencesQuerierComponent,
        WriteAckQuerierComponent,
        AckPacketMessageBuilderComponent,
        AckPacketPayloadBuilderComponent,
        ChannelHandshakeMessageBuilderComponent,
        ChannelHandshakePayloadBuilderComponent,
        ConnectionHandshakeMessageBuilderComponent,
        ConnectionHandshakePayloadBuilderComponent,
        ReceivePacketMessageBuilderComponent,
        ReceivePacketPayloadBuilderComponent,
        TimeoutUnorderedPacketMessageBuilderComponent,
        TimeoutUnorderedPacketPayloadBuilderComponent,
        ClientStateQuerierComponent,
        ConsensusStateHeightQuerierComponent,
        CreateClientMessageBuilderComponent,
        CreateClientPayloadBuilderComponent,
        UpdateClientMessageBuilderComponent,
        UpdateClientPayloadBuilderComponent,
        PacketFromWriteAckBuilderComponent,
    ]: BaseComponents,
);
