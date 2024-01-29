use cgp_core::prelude::delegate_components;
use hermes_relayer_components::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::block_querier::BlockQuerierComponent;
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
use hermes_relayer_components::chain::traits::types::block::{BlockHashComponent, BlockTypeComponent};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::InitChannelOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::connection::{ConnectionHandshakePayloadTypeComponent, InitConnectionOptionsTypeComponent};
use hermes_relayer_components::chain::traits::types::create_client::CreateClientOptionsTypeComponent;
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{GenesisHeightGetterComponent, HeightIncrementerComponent, HeightTypeComponent};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;

use crate::impls::ack_packet_message::BuildCosmosAckPacketMessage;
use crate::impls::ack_packet_payload::BuildCosmosAckPacketPayload;
use crate::impls::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use crate::impls::channel_handshake_payload::BuildCosmosChannelHandshakePayload;
use crate::impls::connection_handshake_message::BuildCosmosConnectionHandshakeMessage;
use crate::impls::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use crate::impls::create_client_message::BuildCosmosCreateClientMessage;
use crate::impls::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::impls::init_channel_options::ProvideCosmosInitChannelOptionsType;
use crate::impls::init_connection_options::ProvideCosmosInitConnectionOptionsType;
use crate::impls::packet_fields::CosmosPacketFieldReader;
use crate::impls::packet_from_ack::BuildCosmosPacketFromWriteAck;
use crate::impls::query_block::QueryCometBlock;
use crate::impls::query_chain_id::QueryChainIdWithChainHandle;
use crate::impls::query_chain_status::QueryChainStatusWithChainHandle;
use crate::impls::query_client_state::QueryCosmosClientStateFromChainHandle;
use crate::impls::query_consensus_state::QueryCosmosConsensusStateFromChainHandle;
use crate::impls::query_consensus_state_height::QueryConsensusStateHeightFromChainHandle;
use crate::impls::query_packet_commitments::QueryCosmosPacketCommitments;
use crate::impls::query_received_packet::QueryReceivedPacketWithChainHandle;
use crate::impls::query_send_packet::QueryCosmosSendPacket;
use crate::impls::query_send_packets::QuerySendPacketsConcurrently;
use crate::impls::query_unreceived_packet::QueryUnreceivedCosmosPacketSequences;
use crate::impls::query_write_ack_event::QueryWriteAckEventFromChainHandle;
use crate::impls::receive_packet_message::BuildCosmosReceivePacketMessage;
use crate::impls::receive_packet_payload::BuildCosmosReceivePacketPayload;
use crate::impls::send_messages_as_tx::SendMessagesToTxContext;
use crate::impls::timeout_packet_message::BuildCosmosTimeoutPacketMessage;
use crate::impls::timeout_packet_payload::BuildCosmosTimeoutPacketPayload;
use crate::impls::types::chain::ProvideCosmosChainTypes;
use crate::impls::types::connection_handshake_payload::ProvideCosmosConnectionHandshakePayloads;
use crate::impls::types::create_client_options::ProvideCosmosCreateClientSettings;
use crate::impls::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::update_client_payload::BuildUpdateClientPayloadWithChainHandle;

pub struct CosmosClientComponents;

delegate_components! {
    #[mark_component(IsCosmosClientComponents)]
    CosmosClientComponents {
        [
            HeightTypeComponent,
            HeightIncrementerComponent,
            GenesisHeightGetterComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            EventTypeComponent,
            IbcChainTypesComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            BlockTypeComponent,
            BlockHashComponent,
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
        ConnectionHandshakePayloadTypeComponent:
            ProvideCosmosConnectionHandshakePayloads,
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
        InitConnectionOptionsTypeComponent:
            ProvideCosmosInitConnectionOptionsType,
        InitChannelOptionsTypeComponent:
            ProvideCosmosInitChannelOptionsType,
        BlockQuerierComponent:
            QueryCometBlock,
    }
}
