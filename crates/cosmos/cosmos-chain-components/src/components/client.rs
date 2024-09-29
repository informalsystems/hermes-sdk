use cgp::core::component::DelegateTo;
use cgp::prelude::*;
pub use hermes_chain_type_components::traits::fields::height::HeightIncrementerComponent;
use hermes_relayer_components::chain::impls::payload_builders::channel::BuildChannelHandshakePayload;
use hermes_relayer_components::chain::impls::payload_builders::connection::BuildConnectionHandshakePayload;
use hermes_relayer_components::chain::impls::payload_builders::packet::BuildPacketPayloads;
use hermes_relayer_components::chain::impls::queries::consensus_state_height::QueryConsensusStateHeightsAndFindHeightBefore;
pub use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
pub use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilderComponent, ChannelOpenConfirmMessageBuilderComponent,
    ChannelOpenInitMessageBuilderComponent, ChannelOpenTryMessageBuilderComponent,
};
pub use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilderComponent, ConnectionOpenConfirmMessageBuilderComponent,
    ConnectionOpenInitMessageBuilderComponent, ConnectionOpenTryMessageBuilderComponent,
};
pub use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
pub use hermes_relayer_components::chain::traits::packet::fields::OutgoingPacketFieldsReaderComponent;
pub use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckBuilderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::channel_handshake::{
    ChannelOpenAckPayloadBuilderComponent, ChannelOpenConfirmPayloadBuilderComponent,
    ChannelOpenTryPayloadBuilderComponent,
};
pub use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenTryPayloadBuilderComponent,
};
pub use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
pub use hermes_relayer_components::chain::traits::queries::ack_packets::{
    AckPacketQuerierComponent, AckPacketsQuerierComponent,
};
pub use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::channel_end::{
    ChannelEndQuerierComponent, ChannelEndWithProofsQuerierComponent,
};
pub use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, AllRawClientStatesQuerierComponent,
    ClientStateQuerierComponent, ClientStateWithProofsQuerierComponent,
    RawClientStateQuerierComponent, RawClientStateWithProofsQuerierComponent,
};
pub use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerierComponent, ConnectionEndWithProofsQuerierComponent,
};
pub use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
    RawConsensusStateQuerierComponent, RawConsensusStateWithProofsQuerierComponent,
};
pub use hermes_relayer_components::chain::traits::queries::consensus_state_height::{
    ConsensusStateHeightQuerierComponent, ConsensusStateHeightsQuerierComponent,
};
pub use hermes_relayer_components::chain::traits::queries::counterparty_chain_id::CounterpartyChainIdQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_acknowledgements::PacketAcknowledgementsQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_commitment::PacketCommitmentQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_commitments::PacketCommitmentsQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_receipt::PacketReceiptQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::send_packets::{
    SendPacketQuerierComponent, SendPacketsQuerierComponent,
};
pub use hermes_relayer_components::chain::traits::queries::unreceived_acks_sequences::UnreceivedAcksSequencesQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerierComponent;
pub use hermes_relayer_components::chain::traits::types::block::{
    BlockHashComponent, BlockTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::chain_id::ChainIdTypeComponent;
pub use hermes_relayer_components::chain::traits::types::channel::{
    ChannelEndTypeComponent, ChannelOpenAckPayloadTypeComponent,
    ChannelOpenConfirmPayloadTypeComponent, ChannelOpenTryPayloadTypeComponent,
    InitChannelOptionsTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::client_state::{
    ClientStateFieldsGetterComponent, ClientStateTypeComponent, RawClientStateTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::connection::{
    ConnectionEndTypeComponent, ConnectionOpenAckPayloadTypeComponent,
    ConnectionOpenConfirmPayloadTypeComponent, ConnectionOpenInitPayloadTypeComponent,
    ConnectionOpenTryPayloadTypeComponent, InitConnectionOptionsTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateFieldComponent, ConsensusStateTypeComponent, RawConsensusStateTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, CreateClientMessageOptionsTypeComponent,
    CreateClientPayloadOptionsTypeComponent, CreateClientPayloadTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::event::EventTypeComponent;
pub use hermes_relayer_components::chain::traits::types::height::{
    GenesisHeightGetterComponent, HeightFieldComponent, HeightTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::ibc::{
    ChannelIdTypeComponent, ClientIdTypeComponent, ConnectionIdTypeComponent,
    CounterpartyMessageHeightGetterComponent, PortIdTypeComponent, SequenceTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    ChannelOpenInitEventComponent, ChannelOpenTryEventComponent,
};
pub use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ConnectionOpenInitEventComponent, ConnectionOpenTryEventComponent,
};
pub use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::SendPacketEventComponent;
pub use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::WriteAckEventComponent;
pub use hermes_relayer_components::chain::traits::types::message::{
    MessageSizeEstimatorComponent, MessageTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::packet::OutgoingPacketTypeComponent;
pub use hermes_relayer_components::chain::traits::types::packets::ack::{
    AckPacketPayloadTypeComponent, AcknowledgementTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::packets::receive::{
    PacketCommitmentTypeComponent, ReceivePacketPayloadTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::packets::timeout::{
    PacketReceiptTypeComponent, TimeoutUnorderedPacketPayloadTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetterComponent, CommitmentProofHeightGetterComponent,
    CommitmentProofTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::status::ChainStatusTypeComponent;
pub use hermes_relayer_components::chain::traits::types::timestamp::TimeMeasurerComponent;
pub use hermes_relayer_components::chain::traits::types::timestamp::{
    TimeTypeComponent, TimeoutTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;

use crate::components::delegate::DelegateCosmosChainComponents;
use crate::impls::channel::init_channel_options::ProvideCosmosInitChannelOptionsType;
use crate::impls::client::create_client_payload::BuildCreateClientPayloadWithChainHandle;
use crate::impls::client::update_client_payload::BuildUpdateClientPayloadWithChainHandle;
use crate::impls::connection::init_connection_options::ProvideCosmosInitConnectionOptionsType;
use crate::impls::events::ProvideCosmosEvents;
use crate::impls::packet::packet_fields::CosmosPacketFieldReader;
use crate::impls::packet::packet_from_ack::BuildCosmosPacketFromWriteAck;
use crate::impls::packet::packet_message::BuildCosmosPacketMessages;
use crate::impls::queries::abci::QueryAbci;
use crate::impls::queries::ack_packet::QueryCosmosAckPacket;
use crate::impls::queries::ack_packets::QueryAckPacketsConcurrently;
use crate::impls::queries::block::QueryCometBlock;
use crate::impls::queries::chain_id::QueryChainIdWithChainHandle;
use crate::impls::queries::chain_status::QueryChainStatusWithChainHandle;
use crate::impls::queries::channel_end::QueryCosmosChannelEndFromAbci;
use crate::impls::queries::client_state::QueryCosmosClientStateFromAbci;
use crate::impls::queries::connection_end::QueryCosmosConnectionEndFromAbci;
use crate::impls::queries::consensus_state::QueryCosmosConsensusStateFromAbci;
use crate::impls::queries::packet_acknowledgement::QueryPacketAcknowledgementFromAbci;
use crate::impls::queries::packet_acknowledgements::QueryCosmosPacketAcknowledgements;
use crate::impls::queries::packet_commitment::QueryPacketCommitmentFromAbci;
use crate::impls::queries::packet_commitments::QueryCosmosPacketCommitments;
use crate::impls::queries::packet_receipt::QueryPacketReceiptFromAbci;
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
pub use crate::traits::abci_query::AbciQuerierComponent;

define_components! {
    CosmosClientComponents {
        [
            HeightTypeComponent,
            HeightFieldComponent,
            HeightIncrementerComponent,
            GenesisHeightGetterComponent,
            TimeTypeComponent,
            TimeMeasurerComponent,
            TimeoutTypeComponent,
            ChainIdTypeComponent,
            MessageTypeComponent,
            MessageSizeEstimatorComponent,
            EventTypeComponent,
            ClientIdTypeComponent,
            ConnectionIdTypeComponent,
            ChannelIdTypeComponent,
            PortIdTypeComponent,
            SequenceTypeComponent,
            ConnectionEndTypeComponent,
            ChannelEndTypeComponent,
            OutgoingPacketTypeComponent,
            ChainStatusTypeComponent,
            BlockTypeComponent,
            BlockHashComponent,
            CommitmentPrefixTypeComponent,
            CommitmentProofTypeComponent,
            CommitmentProofHeightGetterComponent,
            CommitmentProofBytesGetterComponent,
            PacketCommitmentTypeComponent,
            AcknowledgementTypeComponent,
            PacketReceiptTypeComponent,
        ]:
            ProvideCosmosChainTypes,
        [
            CreateClientEventComponent,
            ConnectionOpenInitEventComponent,
            ConnectionOpenTryEventComponent,
            ChannelOpenInitEventComponent,
            ChannelOpenTryEventComponent,
            SendPacketEventComponent,
            WriteAckEventComponent,
        ]:
            ProvideCosmosEvents,
        [
            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            ConnectionOpenInitPayloadTypeComponent,
            ConnectionOpenTryPayloadTypeComponent,
            ConnectionOpenAckPayloadTypeComponent,
            ConnectionOpenConfirmPayloadTypeComponent,
            ChannelOpenTryPayloadTypeComponent,
            ChannelOpenAckPayloadTypeComponent,
            ChannelOpenConfirmPayloadTypeComponent,
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
        [
            ConsensusStateTypeComponent,
            ConsensusStateFieldComponent,
        ]:
            ProvideTendermintConsensusState,
        OutgoingPacketFieldsReaderComponent:
            CosmosPacketFieldReader,
        ConsensusStateHeightQuerierComponent:
            QueryConsensusStateHeightsAndFindHeightBefore,
        WriteAckQuerierComponent:
            QueryWriteAckEventFromChainHandle,
        [
            RawClientStateQuerierComponent,
            RawClientStateWithProofsQuerierComponent,
            AllRawClientStatesQuerierComponent,
        ]:
            QueryCosmosClientStateFromAbci,
        [
            RawConsensusStateQuerierComponent,
            RawConsensusStateWithProofsQuerierComponent,
        ]:
            QueryCosmosConsensusStateFromAbci,
        CreateClientPayloadOptionsTypeComponent:
            ProvideCosmosCreateClientSettings,
        CreateClientPayloadBuilderComponent:
            BuildCreateClientPayloadWithChainHandle,
        UpdateClientPayloadBuilderComponent:
            BuildUpdateClientPayloadWithChainHandle,
        CounterpartyChainIdQuerierComponent:
            QueryChainIdWithChainHandle,

        [
            ConnectionOpenInitPayloadBuilderComponent,
            ConnectionOpenTryPayloadBuilderComponent,
            ConnectionOpenAckPayloadBuilderComponent,
            ConnectionOpenConfirmPayloadBuilderComponent,
        ]:
            BuildConnectionHandshakePayload,
        [
            ChannelOpenTryPayloadBuilderComponent,
            ChannelOpenAckPayloadBuilderComponent,
            ChannelOpenConfirmPayloadBuilderComponent,
        ]:
            BuildChannelHandshakePayload,

        [
            ReceivePacketPayloadBuilderComponent,
            AckPacketPayloadBuilderComponent,
            TimeoutUnorderedPacketPayloadBuilderComponent,
        ]:
            BuildPacketPayloads,

        [
            ReceivePacketMessageBuilderComponent,
            AckPacketMessageBuilderComponent,
        TimeoutUnorderedPacketMessageBuilderComponent,
        ]:
            BuildCosmosPacketMessages,

        PacketCommitmentsQuerierComponent:
            QueryCosmosPacketCommitments,
        ReceivedPacketQuerierComponent:
            QueryReceivedPacketWithChainHandle,

        UnreceivedPacketSequencesQuerierComponent:
            QueryUnreceivedCosmosPacketSequences,
        UnreceivedAcksSequencesQuerierComponent:
            QueryUnreceivedCosmosAcksSequences,

        PacketCommitmentQuerierComponent:
            QueryPacketCommitmentFromAbci,
        PacketAcknowledgementQuerierComponent:
            QueryPacketAcknowledgementFromAbci,
        PacketReceiptQuerierComponent:
            QueryPacketReceiptFromAbci,
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
            ChannelEndQuerierComponent,
            ChannelEndWithProofsQuerierComponent,
        ]:
            QueryCosmosChannelEndFromAbci,

        [
            ConsensusStateHeightsQuerierComponent,
            CounterpartyMessageHeightGetterComponent,

            UpdateClientMessageBuilderComponent,

            CreateClientMessageBuilderComponent,
            CreateClientMessageOptionsTypeComponent,

            ClientStateQuerierComponent,
            ClientStateWithProofsQuerierComponent,
            AllClientStatesQuerierComponent,

            ConsensusStateQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,

            ConnectionOpenInitMessageBuilderComponent,
            ConnectionOpenTryMessageBuilderComponent,
            ConnectionOpenAckMessageBuilderComponent,
            ConnectionOpenConfirmMessageBuilderComponent,

            ChannelOpenInitMessageBuilderComponent,
            ChannelOpenTryMessageBuilderComponent,
            ChannelOpenAckMessageBuilderComponent,
            ChannelOpenConfirmMessageBuilderComponent,
        ]:
            DelegateTo<DelegateCosmosChainComponents>,
    }
}
