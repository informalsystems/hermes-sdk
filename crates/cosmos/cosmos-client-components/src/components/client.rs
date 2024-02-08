use cgp_core::prelude::delegate_components;
use hermes_relayer_components::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::connection_handshake_payload_builder::ConnectionHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use hermes_relayer_components::chain::traits::components::packet_fields_reader::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::components::packet_from_write_ack_builder::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::{
    TimeoutUnorderedPacketMessageBuilderComponent, TimeoutUnorderedPacketPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::ConsensusStateHeightQuerierComponent;
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerierComponent;
use hermes_relayer_components::chain::traits::queries::send_packets::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerierComponent;
use hermes_relayer_components::chain::traits::types::block::{
    BlockHashComponent, BlockTypeComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelHandshakePayloadTypeComponent, InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionHandshakePayloadTypeComponent, InitConnectionOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetterComponent, HeightIncrementerComponent, HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::MessageTypeComponent;
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AckPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::ReceivePacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::TimeoutUnorderedPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;

use crate::impls::ack_packet_message::BuildCosmosAckPacketMessage;
use crate::impls::ack_packet_payload::BuildCosmosAckPacketPayload;
use crate::impls::channel::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use crate::impls::channel::channel_handshake_payload::BuildCosmosChannelHandshakePayload;
use crate::impls::channel::init_channel_options::ProvideCosmosInitChannelOptionsType;
use crate::impls::client::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::client::update_client_payload::BuildUpdateClientPayloadWithChainHandle;
use crate::impls::connection::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use crate::impls::connection::init_connection_options::ProvideCosmosInitConnectionOptionsType;
use crate::impls::packet_fields::CosmosPacketFieldReader;
use crate::impls::packet_from_ack::BuildCosmosPacketFromWriteAck;
use crate::impls::query_block::QueryCometBlock;
use crate::impls::query_chain_id::QueryChainIdWithChainHandle;
use crate::impls::query_chain_status::QueryChainStatusWithChainHandle;
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
use crate::impls::types::create_client_options::ProvideCosmosCreateClientSettings;
use crate::impls::types::payload::ProvideCosmosPayloadTypes;

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
        [
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            ConnectionHandshakePayloadTypeComponent,
            ChannelHandshakePayloadTypeComponent,
            ReceivePacketPayloadTypeComponent,
            AckPacketPayloadTypeComponent,
            TimeoutUnorderedPacketPayloadTypeComponent,
        ]:
            ProvideCosmosPayloadTypes,
        MessageSenderComponent:
            SendMessagesToTxContext,
        PacketFieldsReaderComponent:
            CosmosPacketFieldReader,
        ConsensusStateHeightQuerierComponent:
            QueryConsensusStateHeightFromChainHandle,
        WriteAckQuerierComponent:
            QueryWriteAckEventFromChainHandle,
        CreateClientOptionsTypeComponent:
            ProvideCosmosCreateClientSettings,
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
        InitConnectionOptionsTypeComponent:
            ProvideCosmosInitConnectionOptionsType,
        InitChannelOptionsTypeComponent:
            ProvideCosmosInitChannelOptionsType,
        BlockQuerierComponent:
            QueryCometBlock,
    }
}
