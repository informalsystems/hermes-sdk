use cgp::core::component::UseDelegate;
use cgp::prelude::*;
pub use hermes_chain_type_components::traits::fields::height::HeightIncrementerComponent;
pub use hermes_chain_type_components::traits::fields::message_response_events::MessageResponseEventsGetterComponent;
pub use hermes_chain_type_components::traits::types::message_response::MessageResponseTypeComponent;
use hermes_relayer_components::chain::impls::payload_builders::channel::BuildChannelHandshakePayload;
use hermes_relayer_components::chain::impls::payload_builders::connection::BuildConnectionHandshakePayload;
use hermes_relayer_components::chain::impls::payload_builders::packet::BuildPacketPayloads;
use hermes_relayer_components::chain::impls::queries::block_events::{
    RetryQueryBlockEvents, WaitBlockHeightAndQueryEvents,
};
use hermes_relayer_components::chain::impls::queries::consensus_state_height::QueryConsensusStateHeightsAndFindHeightBefore;
pub use hermes_relayer_components::chain::traits::commitment_prefix::CommitmentPrefixTypeComponent;
pub use hermes_relayer_components::chain::traits::extract_data::{
    EventExtractorComponent, ExtractFromMessageResponseViaEvents, MessageResponseExtractorComponent,
};
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
pub use hermes_relayer_components::chain::traits::packet::fields::{
    PacketDstChannelIdGetterComponent, PacketDstPortIdGetterComponent,
    PacketSequenceGetterComponent, PacketSrcChannelIdGetterComponent,
    PacketSrcPortIdGetterComponent, PacketTimeoutHeightGetterComponent,
    PacketTimeoutTimestampGetterComponent,
};
pub use hermes_relayer_components::chain::traits::packet::filter::{
    IncomingPacketFilterComponent, OutgoingPacketFilterComponent,
};
pub use hermes_relayer_components::chain::traits::packet::from_send_packet::PacketFromSendPacketEventBuilderComponent;
pub use hermes_relayer_components::chain::traits::packet::from_write_ack::PacketFromWriteAckEventBuilderComponent;
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
pub use hermes_relayer_components::chain::traits::queries::block::BlockQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::block_events::BlockEventsQuerierComponent;
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
pub use hermes_relayer_components::chain::traits::queries::packet_commitment::PacketCommitmentQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_is_cleared::PacketIsClearedQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_is_received::PacketIsReceivedQuerierComponent;
pub use hermes_relayer_components::chain::traits::queries::packet_receipt::PacketReceiptQuerierComponent;
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
    ClientStateFieldsComponent, ClientStateTypeComponent, RawClientStateTypeComponent,
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
pub use hermes_relayer_components::chain::traits::types::timestamp::{
    TimeMeasurerComponent, TimeTypeComponent, TimeoutTypeComponent,
};
pub use hermes_relayer_components::chain::traits::types::update_client::UpdateClientPayloadTypeComponent;

use crate::components::delegate::DelegateCosmosChainComponents;
use crate::impls::channel::init_channel_options::ProvideCosmosInitChannelOptionsType;
use crate::impls::connection::init_connection_options::ProvideCosmosInitConnectionOptionsType;
use crate::impls::events::ProvideCosmosEvents;
use crate::impls::packet::packet_message::BuildCosmosPacketMessages;
use crate::impls::queries::abci::QueryAbci;
use crate::impls::queries::block::QueryCometBlock;
use crate::impls::queries::block_events::QueryCosmosBlockEvents;
use crate::impls::queries::chain_id::QueryChainIdFromAbci;
use crate::impls::queries::chain_status::QueryCosmosChainStatus;
use crate::impls::queries::channel_end::QueryCosmosChannelEndFromAbci;
use crate::impls::queries::client_state::QueryCosmosClientStateFromAbci;
use crate::impls::queries::connection_end::QueryCosmosConnectionEndFromAbci;
use crate::impls::queries::consensus_state::QueryCosmosConsensusStateFromAbci;
use crate::impls::queries::packet_acknowledgement::QueryPacketAcknowledgementFromAbci;
use crate::impls::queries::packet_commitment::QueryPacketCommitmentFromAbci;
use crate::impls::queries::packet_receipt::QueryPacketReceiptFromAbci;
use crate::impls::queries::received_ack::QueryCosmosPacketIsCleared;
use crate::impls::queries::received_packet::QueryCosmosPacketIsReceived;
use crate::impls::queries::write_ack_event::QueryCosmosWriteAckEvent;
use crate::impls::relay::packet_filter::FilterPacketWithConfig;
use crate::impls::types::chain::ProvideCosmosChainTypes;
use crate::impls::types::client_state::ProvideAnyRawClientState;
use crate::impls::types::consensus_state::ProvideAnyRawConsensusState;
use crate::impls::types::payload::ProvideCosmosPayloadTypes;
use crate::impls::unbonding_period::StakingParamsUnbondingPeriod;
pub use crate::traits::abci_query::AbciQuerierComponent;
pub use crate::traits::unbonding_period::UnbondingPeriodQuerierComponent;

cgp_preset! {
    CosmosChainClientPreset {
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
            MessageResponseTypeComponent,
            MessageResponseEventsGetterComponent,
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
            EventExtractorComponent,
            PacketFromSendPacketEventBuilderComponent,
            PacketFromWriteAckEventBuilderComponent,
        ]:
            ProvideCosmosEvents,
        [
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
        MessageResponseExtractorComponent:
            ExtractFromMessageResponseViaEvents,
        RawClientStateTypeComponent:
            ProvideAnyRawClientState,
        RawConsensusStateTypeComponent:
            ProvideAnyRawConsensusState,
        ConsensusStateHeightQuerierComponent:
            QueryConsensusStateHeightsAndFindHeightBefore,
        WriteAckQuerierComponent:
            QueryCosmosWriteAckEvent,
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
        CounterpartyChainIdQuerierComponent:
            QueryChainIdFromAbci,
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
            AckPacketMessageBuilderComponent,
            TimeoutUnorderedPacketMessageBuilderComponent,
        ]:
            BuildCosmosPacketMessages,

        PacketIsReceivedQuerierComponent:
            QueryCosmosPacketIsReceived,
        PacketIsClearedQuerierComponent:
            QueryCosmosPacketIsCleared,

        PacketCommitmentQuerierComponent:
            QueryPacketCommitmentFromAbci,
        PacketAcknowledgementQuerierComponent:
            QueryPacketAcknowledgementFromAbci,
        PacketReceiptQuerierComponent:
            QueryPacketReceiptFromAbci,
        ChainStatusQuerierComponent:
            QueryCosmosChainStatus,
        InitConnectionOptionsTypeComponent:
            ProvideCosmosInitConnectionOptionsType,
        InitChannelOptionsTypeComponent:
            ProvideCosmosInitChannelOptionsType,
        BlockQuerierComponent:
            QueryCometBlock,
        BlockEventsQuerierComponent:
            RetryQueryBlockEvents<
                5,
                WaitBlockHeightAndQueryEvents<
                    QueryCosmosBlockEvents
                >>,
        AbciQuerierComponent:
            QueryAbci,
        UnbondingPeriodQuerierComponent:
            StakingParamsUnbondingPeriod,
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
            OutgoingPacketFilterComponent,
            IncomingPacketFilterComponent,
        ]:
            FilterPacketWithConfig<symbol!("packet_filter")>,
        [
            ClientStateTypeComponent,
            ClientStateFieldsComponent,

            ConsensusStateTypeComponent,
            ConsensusStateFieldComponent,

            CreateClientPayloadTypeComponent,
            UpdateClientPayloadTypeComponent,
            CreateClientPayloadOptionsTypeComponent,

            ConsensusStateHeightsQuerierComponent,
            CounterpartyMessageHeightGetterComponent,

            UpdateClientMessageBuilderComponent,

            CreateClientMessageBuilderComponent,
            CreateClientMessageOptionsTypeComponent,

            CreateClientPayloadBuilderComponent,
            UpdateClientPayloadBuilderComponent,

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

            ReceivePacketMessageBuilderComponent,

            PacketSrcChannelIdGetterComponent,
            PacketSrcPortIdGetterComponent,
            PacketDstChannelIdGetterComponent,
            PacketDstPortIdGetterComponent,
            PacketSequenceGetterComponent,
            PacketTimeoutHeightGetterComponent,
            PacketTimeoutTimestampGetterComponent,
        ]:
            UseDelegate<DelegateCosmosChainComponents>,
    }
}
