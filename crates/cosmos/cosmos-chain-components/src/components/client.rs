use cgp_core::prelude::delegate_components;
use hermes_relayer_components::chain::impls::delegate::message_builders::channel_handshake::DelegateBuildChannelHandshakeMessage;
use hermes_relayer_components::chain::impls::delegate::message_builders::connection_handshake::DelegateBuildConnectionHandshakeMessage;
use hermes_relayer_components::chain::impls::delegate::message_builders::create_client::DelegateBuildCreateClientMessage;
use hermes_relayer_components::chain::impls::delegate::message_builders::update_client::DelegateBuildUpdateClientMessage;
use hermes_relayer_components::chain::impls::delegate::queries::client_state::DelegateQueryClientState;
use hermes_relayer_components::chain::impls::delegate::queries::consensus_state::DelegateQueryConsensusState;
use hermes_relayer_components::chain::impls::delegate::queries::consensus_state_heights::DelegateQueryConsensusStateHeights;
use hermes_relayer_components::chain::impls::queries::consensus_state_height::QueryConsensusStateHeightsAndFindHeightBefore;
use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
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
use hermes_relayer_components::chain::traits::queries::ack_packets::{
    AckPacketQuerierComponent, AckPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, AllRawClientStatesQuerierComponent,
    ClientStateQuerierComponent, RawClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerierComponent, ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, RawConsensusStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    ConsensusStateHeightQuerierComponent, ConsensusStateHeightsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerierComponent;
use hermes_relayer_components::chain::traits::queries::send_packets::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::unreceived_acks_sequences::UnreceivedAcksSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerierComponent;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerierComponent;
use hermes_relayer_components::chain::traits::types::block::{
    BlockHashComponent, BlockTypeComponent,
};
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
use hermes_relayer_components::chain::traits::types::channel::{
    ChannelHandshakePayloadTypeComponent, InitChannelOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent, RawClientStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ConnectionHandshakePayloadTypeComponent,
    InitConnectionOptionsTypeComponent,
};
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateTypeComponent, RawConsensusStateTypeComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetterComponent, HeightFieldComponent, HeightIncrementerComponent,
    HeightTypeComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::IbcChainTypesComponent;
use hermes_relayer_components::chain::traits::types::message::{
    MessageSizeEstimatorComponent, MessageTypeComponent,
};
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProviderComponent;
use hermes_relayer_components::chain::traits::types::packets::ack::AckPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::receive::ReceivePacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::packets::timeout::TimeoutUnorderedPacketPayloadTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
use hermes_relayer_components::chain::traits::types::timestamp::TimestampTypeComponent;
use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;

use crate::components::delegate::DelegateCosmosChainComponents;
use crate::impls::channel::channel_handshake_payload::BuildCosmosChannelHandshakePayload;
use crate::impls::channel::init_channel_options::ProvideCosmosInitChannelOptionsType;
use crate::impls::client::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::impls::client::update_client_payload::BuildUpdateClientPayloadWithChainHandle;
use crate::impls::connection::connection_handshake_payload::BuildCosmosConnectionHandshakePayload;
use crate::impls::connection::init_connection_options::ProvideCosmosInitConnectionOptionsType;
use crate::impls::events::ProvideCosmosEvents;
use crate::impls::packet::ack_packet_message::BuildCosmosAckPacketMessage;
use crate::impls::packet::ack_packet_payload::BuildCosmosAckPacketPayload;
use crate::impls::packet::packet_fields::CosmosPacketFieldReader;
use crate::impls::packet::packet_from_ack::BuildCosmosPacketFromWriteAck;
use crate::impls::packet::receive_packet_message::BuildCosmosReceivePacketMessage;
use crate::impls::packet::receive_packet_payload::BuildCosmosReceivePacketPayload;
use crate::impls::packet::timeout_packet_message::BuildCosmosTimeoutPacketMessage;
use crate::impls::packet::timeout_packet_payload::BuildCosmosTimeoutPacketPayload;
use crate::impls::queries::abci::QueryAbci;
use crate::impls::queries::ack_packet::QueryCosmosAckPacket;
use crate::impls::queries::ack_packets::QueryAckPacketsConcurrently;
use crate::impls::queries::block::QueryCometBlock;
use crate::impls::queries::chain_id::QueryChainIdWithChainHandle;
use crate::impls::queries::chain_status::QueryChainStatusWithChainHandle;
use crate::impls::queries::client_state::QueryCosmosClientStateFromAbci;
use crate::impls::queries::connection_end::QueryCosmosConnectionEndFromAbci;
use crate::impls::queries::consensus_state::QueryCosmosConsensusStateFromAbci;
use crate::impls::queries::packet_acknowledgements::QueryCosmosPacketAcknowledgements;
use crate::impls::queries::packet_commitments::QueryCosmosPacketCommitments;
use crate::impls::queries::received_packet::QueryReceivedPacketWithChainHandle;
use crate::impls::queries::send_packet::QueryCosmosSendPacket;
use crate::impls::queries::send_packets::QuerySendPacketsConcurrently;
use crate::impls::queries::unreceived_acks::QueryUnreceivedCosmosAcksSequences;
use crate::impls::queries::unreceived_packet::QueryUnreceivedCosmosPacketSequences;
use crate::impls::queries::write_ack_event::QueryWriteAckEventFromChainHandle;
use crate::impls::types::chain::ProvideCosmosChainTypes;
use crate::impls::types::client_state::{ProvideAnyRawClientState, ProvideTendermintClientState};
use crate::impls::types::consensus_state::{
    ProvideAnyRawConsensusState, ProvideTendermintConsensusState,
};
use crate::impls::types::create_client_options::ProvideCosmosCreateClientSettings;
use crate::impls::types::payload::ProvideCosmosPayloadTypes;
use crate::traits::abci_query::AbciQuerierComponent;

pub struct CosmosClientComponents;

delegate_components! {
    #[mark_component(IsCosmosClientComponents)]
    CosmosClientComponents {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            GenesisHeightGetterComponent,
            TimestampTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            MessageSizeEstimatorComponent,
            EventTypeComponent,
            IbcChainTypesComponent,
            ConnectionEndTypeComponent,
            IbcPacketTypesProviderComponent,
            ChainStatusTypeComponent,
            BlockTypeComponent,
            BlockHashComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            CreateClientEventComponent,
        ]:
            ProvideCosmosEvents,
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
        [
            ClientStateTypeComponent,
            ClientStateFieldsGetterComponent,
        ]:
            ProvideTendermintClientState,
        RawClientStateTypeComponent:
            ProvideAnyRawClientState,
        RawConsensusStateTypeComponent:
            ProvideAnyRawConsensusState,
        ConsensusStateTypeComponent:
            ProvideTendermintConsensusState,
        PacketFieldsReaderComponent:
            CosmosPacketFieldReader,
        ConsensusStateHeightQuerierComponent:
            QueryConsensusStateHeightsAndFindHeightBefore,
        WriteAckQuerierComponent:
            QueryWriteAckEventFromChainHandle,
        [
            RawClientStateQuerierComponent,
            AllRawClientStatesQuerierComponent,
        ]:
            QueryCosmosClientStateFromAbci,
        RawConsensusStateQuerierComponent:
            QueryCosmosConsensusStateFromAbci,
        CreateClientOptionsTypeComponent:
            ProvideCosmosCreateClientSettings,
        CreateClientPayloadBuilderComponent:
            BuildCreateClientPayloadWithChainHandle,
        UpdateClientPayloadBuilderComponent:
            BuildUpdateClientPayloadWithChainHandle,
        CounterpartyChainIdQuerierComponent:
            QueryChainIdWithChainHandle,
        ConnectionHandshakePayloadBuilderComponent:
            BuildCosmosConnectionHandshakePayload,
        ChannelHandshakePayloadBuilderComponent:
            BuildCosmosChannelHandshakePayload,
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
        UnreceivedAcksSequencesQuerierComponent:
            QueryUnreceivedCosmosAcksSequences,
        PacketAcknowledgementsQuerierComponent:
            QueryCosmosPacketAcknowledgements,
        SendPacketQuerierComponent:
            QueryCosmosSendPacket,
        SendPacketsQuerierComponent:
            QuerySendPacketsConcurrently,
        AckPacketQuerierComponent:
            QueryCosmosAckPacket,
        AckPacketsQuerierComponent:
            QueryAckPacketsConcurrently,
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
        [
            ConnectionEndQuerierComponent,
            ConnectionEndWithProofsQuerierComponent,
        ]:
            QueryCosmosConnectionEndFromAbci,
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
        ]:
            DelegateQueryClientState<DelegateCosmosChainComponents>,
        CreateClientMessageBuilderComponent:
            DelegateBuildCreateClientMessage<DelegateCosmosChainComponents>,
        ConsensusStateQuerierComponent:
            DelegateQueryConsensusState<DelegateCosmosChainComponents>,
        ConsensusStateHeightsQuerierComponent:
            DelegateQueryConsensusStateHeights<DelegateCosmosChainComponents>,
        UpdateClientMessageBuilderComponent:
            DelegateBuildUpdateClientMessage<DelegateCosmosChainComponents>,
        ConnectionHandshakeMessageBuilderComponent:
            DelegateBuildConnectionHandshakeMessage<DelegateCosmosChainComponents>,
        ChannelHandshakeMessageBuilderComponent:
            DelegateBuildChannelHandshakeMessage<DelegateCosmosChainComponents>,
    }
}
