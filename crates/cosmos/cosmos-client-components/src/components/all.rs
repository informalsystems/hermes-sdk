use cgp_core::prelude::delegate_components;
use hermes_relayer_components::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::chain_status_querier::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerierComponent;
use hermes_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::consensus_state_height_querier::ConsensusStateHeightQuerierComponent;
use hermes_relayer_components::chain::traits::components::consensus_state_querier::ConsensusStateQuerierComponent;
use hermes_relayer_components::chain::traits::components::counterparty_chain_id_querier::CounterpartyChainIdQuerierComponent;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use hermes_relayer_components::chain::traits::components::packet_commitments_querier::PacketCommitmentsQuerierComponent;
use hermes_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::components::packet_from_write_ack_builder::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::received_packet_querier::ReceivedPacketQuerierComponent;
use hermes_relayer_components::chain::traits::components::send_packets_querier::{SendPacketQuerierComponent, SendPacketsQuerierComponent};
use hermes_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{TimeoutUnorderedPacketPayloadBuilderComponent, TimeoutUnorderedPacketMessageBuilderComponent};
use hermes_relayer_components::chain::traits::components::unreceived_packet_sequences_querier::UnreceivedPacketSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::write_ack_querier::WriteAckQuerierComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::create_client::CreateClientOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::height::HeightTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeProviderComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeProviderComponent;

use crate::components::ack_packet_message::BuildCosmosAckPacketMessage;
use crate::components::ack_packet_payload::BuildCosmosAckPacketPayload;
use crate::components::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use crate::components::channel_handshake_payload::BuildCosmosChannelHandshakePayload;
use crate::components::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::components::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use crate::components::create_client_message::BuildCosmosCreateClientMessage;
use crate::components::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::components::packet_fields::CosmosPacketFieldReader;
use crate::components::packet_from_ack::BuildCosmosPacketFromWriteAck;
use crate::components::query_chain_id::QueryChainIdWithChainHandle;
use crate::components::query_chain_status::QueryChainStatusWithChainHandle;
use crate::components::query_client_state::QueryCosmosClientStateFromChainHandle;
use crate::components::query_consensus_state::QueryCosmosConsensusStateFromChainHandle;
use crate::components::query_consensus_state_height::QueryConsensusStateHeightFromChainHandle;
use crate::components::query_packet_commitments::QueryCosmosPacketCommitments;
use crate::components::query_received_packet::QueryReceivedPacketWithChainHandle;
use crate::components::query_send_packet::QueryCosmosSendPacket;
use crate::components::query_send_packets::QuerySendPacketsConcurrently;
use crate::components::query_unreceived_packet::QueryUnreceivedCosmosPacketSequences;
use crate::components::query_write_ack_event::QueryWriteAckEventFromChainHandle;
use crate::components::receive_packet_message::BuildCosmosReceivePacketMessage;
use crate::components::receive_packet_payload::BuildCosmosReceivePacketPayload;
use crate::components::send_messages_as_tx::SendMessagesToTxContext;
use crate::components::timeout_packet_message::BuildCosmosTimeoutPacketMessage;
use crate::components::timeout_packet_payload::BuildCosmosTimeoutPacketPayload;
use crate::components::types::chain::ProvideCosmosChainTypes;
use crate::components::types::create_client_options::ProvideCosmosCreateClientSettings;
use crate::components::update_client_message::BuildCosmosUpdateClientMessage;
use crate::components::update_client_payload::BuildUpdateClientPayloadWithChainHandle;

pub struct CosmosClientComponents;

delegate_components! {
    CosmosClientComponents {
        [
            HeightTypeProviderComponent,
            TimestampTypeProviderComponent,
            ChainIdTypeProviderComponent,
            MessageTypeProviderComponent,
            EventTypeProviderComponent,
            IbcChainTypesProviderComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeProviderComponent,
        ]:
            ProvideCosmosChainTypes,
        MessageSenderComponent:
            SendMessagesToTxContext,
        PacketFieldsReaderComponent:
            CosmosPacketFieldReader,
        ClientStateQuerierComponent:
            QueryCosmosClientStateFromChainHandle,
        ConsensusStateHeightQuerierComponent:
            QueryConsensusStateHeightFromChainHandle,
        WriteAckQuerierComponent:
            QueryWriteAckEventFromChainHandle,
        CreateClientOptionsTypeComponent:
            ProvideCosmosCreateClientSettings,
        CreateClientMessageBuilderComponent:
            BuildCosmosCreateClientMessage,
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
            BuildCosmosConnectionHandshakeMessage,
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
        ChainStatusQuerierComponent:
            QueryChainStatusWithChainHandle,
        ConsensusStateQuerierComponent:
            QueryCosmosConsensusStateFromChainHandle,
    }
}
