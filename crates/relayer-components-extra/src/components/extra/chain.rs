use core::marker::PhantomData;

use cgp_core::{delegate_component, delegate_components};
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
use ibc_relayer_components::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilderComponent;
use ibc_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use ibc_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
use ibc_relayer_components::chain::traits::components::send_packets_querier::SendPacketsQuerierComponent;
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use ibc_relayer_components::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerierComponent;
use ibc_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
use ibc_relayer_components::components::default::chain::DefaultChainComponents;

use crate::telemetry::components::consensus_state::ConsensusStateTelemetryQuerier;
use crate::telemetry::components::status::ChainStatusTelemetryQuerier;

pub struct ExtraChainComponents<BaseComponents>(pub PhantomData<BaseComponents>);

delegate_component!(
    ChainStatusQuerierComponent,
    ExtraChainComponents<BaseComponents>,
    ChainStatusTelemetryQuerier<BaseComponents>,
);

delegate_component!(
    ConsensusStateQuerierComponent,
    ExtraChainComponents<BaseComponents>,
    ConsensusStateTelemetryQuerier<BaseComponents>,
);

delegate_components!(
    [
        MessageSenderComponent,
        PacketFieldsReaderComponent,
        CounterpartyChainIdQuerierComponent,
        PacketCommitmentsQuerierComponent,
        ReceivedPacketQuerierComponent,
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
    ],
    ExtraChainComponents<BaseComponents>,
    DefaultChainComponents<BaseComponents>,
);
