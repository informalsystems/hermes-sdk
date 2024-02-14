use cgp_core::prelude::delegate_components;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReaderComponent;
use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::ChannelHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
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
use hermes_relayer_components::chain::traits::send_message::MessageSenderComponent;
use hermes_relayer_components::chain::traits::types::block::{
    BlockHashComponent, BlockTypeComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelHandshakePayloadTypeComponent, InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::ClientStateTypeComponent;
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

use crate::impls::channel::channel_handshake_message::BuildCosmosChannelHandshakeMessage;
use crate::impls::channel::channel_handshake_payload::BuildCosmosChannelHandshakePayload;
use crate::impls::channel::init_channel_options::ProvideCosmosInitChannelOptionsType;
use crate::impls::client::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::impls::client::update_client_message::BuildCosmosUpdateClientMessage;
use crate::impls::client::update_client_payload::BuildUpdateClientPayloadWithChainHandle;
use crate::impls::connection::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use crate::impls::connection::init_connection_options::ProvideCosmosInitConnectionOptionsType;
use crate::impls::packet::ack_packet_message::BuildCosmosAckPacketMessage;
use crate::impls::packet::ack_packet_payload::BuildCosmosAckPacketPayload;
use crate::impls::packet::packet_fields::CosmosPacketFieldReader;
use crate::impls::packet::packet_from_ack::BuildCosmosPacketFromWriteAck;
use crate::impls::packet::receive_packet_message::BuildCosmosReceivePacketMessage;
use crate::impls::packet::receive_packet_payload::BuildCosmosReceivePacketPayload;
use crate::impls::packet::timeout_packet_message::BuildCosmosTimeoutPacketMessage;
use crate::impls::packet::timeout_packet_payload::BuildCosmosTimeoutPacketPayload;
use crate::impls::queries::abci::QueryAbci;
use crate::impls::queries::block::QueryCometBlock;
use crate::impls::queries::chain_id::QueryChainIdWithChainHandle;
use crate::impls::queries::chain_status::QueryChainStatusWithChainHandle;
use crate::impls::queries::consensus_state_height::QueryConsensusStateHeightFromChainHandle;
use crate::impls::queries::packet_commitments::QueryCosmosPacketCommitments;
use crate::impls::queries::received_packet::QueryReceivedPacketWithChainHandle;
use crate::impls::queries::send_packet::QueryCosmosSendPacket;
use crate::impls::queries::send_packets::QuerySendPacketsConcurrently;
use crate::impls::queries::unreceived_packet::QueryUnreceivedCosmosPacketSequences;
use crate::impls::queries::write_ack_event::QueryWriteAckEventFromChainHandle;
use crate::impls::send_messages_as_tx::SendMessagesToTxContext;
use crate::impls::types::chain::ProvideCosmosChainTypes;
use crate::impls::types::client_state::ProvideTendermintClientState;
use crate::impls::types::create_client_options::ProvideCosmosCreateClientSettings;
use crate::impls::types::payload::ProvideCosmosPayloadTypes;
use crate::traits::abci_query::AbciQuerierComponent;

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
        ClientStateTypeComponent:
            ProvideTendermintClientState,
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
        AbciQuerierComponent:
            QueryAbci,
    }
}
