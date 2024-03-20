//! The following types are used for the OfaChainTypes implementation:
//! * For the Height, a wrapper around a u128, referred to as MockHeight.
//! * For the Timestamp, a u128 representing milliseconds is retrieved
//!   using a shared clock, MockClock.
//! * For messages, an enum, MockMessage, which identifies
//!   RecvPacket, AckPacket, TimeoutPacket, and UpdateClient messages.
//! * The ConsensusState is a set of 4 HashSets used to store which messages
//!   have been sent, received, acknowledged, and timed out.
//! * The ChainStatus is a ConsensusState with a Height and a Timestamp.

use core::time::Duration;

use cgp_core::prelude::*;
use cgp_core::{ErrorRaiserComponent, ErrorTypeComponent};
use eyre::eyre;
use hermes_relayer_components::chain::traits::logs::event::CanLogChainEvent;
use hermes_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilder;
use hermes_relayer_components::chain::traits::packet::fields::PacketFieldsReader;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilder;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilder;
use hermes_relayer_components::chain::traits::payload_builders::timeout_unordered_packet::TimeoutUnorderedPacketPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerier;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerier;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerier;
use hermes_relayer_components::chain::traits::queries::write_ack::WriteAckQuerier;
use hermes_relayer_components::chain::traits::send_message::MessageSender;
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ProvideChainIdType,
};
use hermes_relayer_components::chain::traits::types::client_state::ProvideClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::ProvideConsensusStateType;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::height::{
    HeightIncrementer, ProvideHeightType,
};
use hermes_relayer_components::chain::traits::types::ibc::{
    HasCounterpartyMessageHeight, ProvideIbcChainTypes,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::message::{
    CanEstimateMessageSize, ProvideMessageType,
};
use hermes_relayer_components::chain::traits::types::packet::IbcPacketTypesProvider;
use hermes_relayer_components::chain::traits::types::packets::ack::ProvideAckPacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::receive::ProvideReceivePacketPayloadType;
use hermes_relayer_components::chain::traits::types::packets::timeout::ProvideTimeoutUnorderedPacketPayloadType;
use hermes_relayer_components::chain::traits::types::status::ProvideChainStatusType;
use hermes_relayer_components::chain::traits::types::timestamp::ProvideTimestampType;
use hermes_relayer_components::runtime::traits::runtime::{ProvideRuntime, ProvideRuntimeType};
use hermes_relayer_runtime::types::log::value::LogValue;

use crate::relayer_mock::base::error::{BaseError, Error};
use crate::relayer_mock::base::impls::error::HandleMockError;
use crate::relayer_mock::base::types::aliases::{
    ChainStatus, ChannelId, ClientId, ConsensusState, MockTimestamp, PortId, Sequence,
};
use crate::relayer_mock::base::types::chain::MockChainStatus;
use crate::relayer_mock::base::types::events::{Event, SendPacketEvent, WriteAckEvent};
use crate::relayer_mock::base::types::height::Height as MockHeight;
use crate::relayer_mock::base::types::message::Message as MockMessage;
use crate::relayer_mock::base::types::packet::PacketKey;
use crate::relayer_mock::base::types::runtime::MockRuntimeContext;
use crate::relayer_mock::components::chain::MockChainComponents;
use crate::relayer_mock::contexts::chain::MockChainContext;

impl HasComponents for MockChainContext {
    type Components = MockChainComponents;
}

delegate_components! {
    MockChainComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleMockError,
    }
}

impl ProvideRuntimeType<MockChainContext> for MockChainComponents {
    type Runtime = MockRuntimeContext;
}

impl ProvideRuntime<MockChainContext> for MockChainComponents {
    fn runtime(chain: &MockChainContext) -> &MockRuntimeContext {
        &chain.runtime
    }
}

impl ProvideHeightType<MockChainContext> for MockChainComponents {
    type Height = MockHeight;
}

impl ProvideEventType<MockChainContext> for MockChainComponents {
    type Event = Event;
}

impl ProvideTimestampType<MockChainContext> for MockChainComponents {
    type Timestamp = MockTimestamp;

    fn timestamp_from_nanos(nanos: u64) -> Self::Timestamp {
        MockTimestamp(u128::from(nanos))
    }

    fn timestamp_duration_since(
        earlier: &MockTimestamp,
        later: &MockTimestamp,
    ) -> Option<Duration> {
        later.duration_since(earlier)
    }
}

impl ProvideMessageType<MockChainContext> for MockChainComponents {
    type Message = MockMessage;
}

impl ProvideChainIdType<MockChainContext> for MockChainComponents {
    type ChainId = String;
}

impl ProvideIbcChainTypes<MockChainContext, MockChainContext> for MockChainComponents {
    type ClientId = ClientId;

    type ConnectionId = String;

    type ChannelId = ChannelId;

    type PortId = PortId;

    type Sequence = Sequence;
}

impl IbcPacketTypesProvider<MockChainContext, MockChainContext> for MockChainComponents {
    type IncomingPacket = PacketKey;

    type OutgoingPacket = PacketKey;
}

impl PacketFieldsReader<MockChainContext, MockChainContext> for MockChainComponents {
    fn incoming_packet_src_channel_id(packet: &PacketKey) -> &ChannelId {
        &packet.src_channel_id
    }

    fn incoming_packet_src_port(packet: &PacketKey) -> &PortId {
        &packet.src_port_id
    }

    fn incoming_packet_dst_port(packet: &PacketKey) -> &PortId {
        &packet.dst_port_id
    }

    fn incoming_packet_dst_channel_id(packet: &PacketKey) -> &ChannelId {
        &packet.dst_channel_id
    }

    fn incoming_packet_sequence(packet: &PacketKey) -> &Sequence {
        &packet.sequence
    }

    fn incoming_packet_timeout_height(packet: &PacketKey) -> Option<&MockHeight> {
        Some(&packet.timeout_height)
    }

    fn incoming_packet_timeout_timestamp(packet: &PacketKey) -> &MockTimestamp {
        &packet.timeout_timestamp
    }

    fn outgoing_packet_src_channel_id(packet: &PacketKey) -> &ChannelId {
        &packet.src_channel_id
    }

    fn outgoing_packet_src_port(packet: &PacketKey) -> &PortId {
        &packet.src_port_id
    }

    fn outgoing_packet_dst_port(packet: &PacketKey) -> &PortId {
        &packet.dst_port_id
    }

    fn outgoing_packet_dst_channel_id(packet: &PacketKey) -> &ChannelId {
        &packet.dst_channel_id
    }

    fn outgoing_packet_sequence(packet: &PacketKey) -> &Sequence {
        &packet.sequence
    }

    fn outgoing_packet_timeout_height(packet: &PacketKey) -> Option<&MockHeight> {
        Some(&packet.timeout_height)
    }

    fn outgoing_packet_timeout_timestamp(packet: &PacketKey) -> &MockTimestamp {
        &packet.timeout_timestamp
    }
}

impl HasWriteAckEvent<MockChainContext> for MockChainContext {
    type WriteAckEvent = WriteAckEvent;

    fn try_extract_write_ack_event(event: &Self::Event) -> Option<Self::WriteAckEvent> {
        match event {
            Event::WriteAcknowledgment(h) => Some(WriteAckEvent::new(*h)),
            _ => None,
        }
    }
}

impl ProvideConsensusStateType<MockChainContext, MockChainContext> for MockChainComponents {
    type ConsensusState = ConsensusState;
}

impl ProvideClientStateType<MockChainContext, MockChainContext> for MockChainComponents {
    // TODO
    type ClientState = ();
}

impl ProvideChainStatusType<MockChainContext> for MockChainComponents {
    type ChainStatus = ChainStatus;

    fn chain_status_height(status: &Self::ChainStatus) -> &MockHeight {
        &status.height
    }

    fn chain_status_timestamp(status: &Self::ChainStatus) -> &MockTimestamp {
        &status.timestamp
    }
}

impl HasSendPacketEvent<MockChainContext> for MockChainContext {
    type SendPacketEvent = SendPacketEvent;

    fn try_extract_send_packet_event(event: &Self::Event) -> Option<Self::SendPacketEvent> {
        match event {
            Event::SendPacket(send_packet_event) => Some(send_packet_event.clone()),
            _ => None,
        }
    }

    fn extract_packet_from_send_packet_event(
        event: &Self::SendPacketEvent,
    ) -> Self::OutgoingPacket {
        PacketKey::from(event.clone())
    }
}

impl CanLogChainEvent for MockChainContext {
    fn log_event<'a>(event: &Event) -> LogValue<'_> {
        LogValue::Debug(event)
    }
}

impl HeightIncrementer<MockChainContext> for MockChainComponents {
    fn increment_height(height: &MockHeight) -> Result<MockHeight, Error> {
        Ok(height.increment())
    }
}

impl CanEstimateMessageSize for MockChainContext {
    fn estimate_message_size(_message: &Self::Message) -> Result<usize, Self::Error> {
        // Only single messages are sent by the Mock Chain
        Ok(1)
    }
}

impl ChainIdGetter<MockChainContext> for MockChainComponents {
    fn chain_id(chain: &MockChainContext) -> &String {
        &chain.name
    }
}

#[async_trait]
impl MessageSender<MockChainContext> for MockChainComponents {
    async fn send_messages(
        chain: &MockChainContext,
        messages: Vec<MockMessage>,
    ) -> Result<Vec<Vec<Event>>, Error> {
        chain.process_messages(messages)
    }
}

#[async_trait]
impl ChainStatusQuerier<MockChainContext> for MockChainComponents {
    async fn query_chain_status(chain: &MockChainContext) -> Result<ChainStatus, Error> {
        let height = chain.get_current_height();
        let state = chain.get_current_state();
        // Since the MockChain only updates manually, the Height is increased by
        // 1 everytime the chain status is queried, without changing its state.
        chain.new_block()?;
        let time = chain.runtime.get_time();
        Ok(MockChainStatus::from((height, time, state)))
    }
}

impl CanLogChainPacket<MockChainContext> for MockChainContext {
    fn log_incoming_packet(packet: &PacketKey) -> LogValue<'_> {
        LogValue::Display(packet)
    }

    fn log_outgoing_packet(packet: &PacketKey) -> LogValue<'_> {
        LogValue::Display(packet)
    }
}

impl HasCounterpartyMessageHeight<MockChainContext> for MockChainContext {
    fn counterparty_message_height_for_update_client(message: &MockMessage) -> Option<MockHeight> {
        match message {
            MockMessage::RecvPacket(h, _) => Some(h.increment()),
            MockMessage::AckPacket(h, _) => Some(h.increment()),
            MockMessage::TimeoutPacket(h, _) => Some(h.increment()),
            _ => None,
        }
    }
}

#[async_trait]
impl ConsensusStateQuerier<MockChainContext, MockChainContext> for MockChainComponents {
    async fn query_consensus_state(
        chain: &MockChainContext,
        client_id: &ClientId,
        consensus_height: &MockHeight,
        _query_height: &MockHeight,
    ) -> Result<ConsensusState, Error> {
        let client_consensus =
            chain.query_consensus_state_at_height(client_id.to_string(), *consensus_height)?;
        Ok(client_consensus)
    }
}

#[async_trait]
impl ClientStateQuerier<MockChainContext, MockChainContext> for MockChainComponents {
    async fn query_client_state(
        _chain: &MockChainContext,
        _client_id: &ClientId,
        _height: &MockHeight,
    ) -> Result<(), Error> {
        Ok(())
    }
}

#[async_trait]
impl ReceivedPacketQuerier<MockChainContext, MockChainContext> for MockChainComponents {
    async fn query_packet_is_received(
        chain: &MockChainContext,
        port_id: &PortId,
        channel_id: &ChannelId,
        sequence: &Sequence,
    ) -> Result<bool, Error> {
        let state = chain.get_current_state();
        Ok(state.check_received((port_id.clone(), channel_id.clone(), *sequence)))
    }
}

#[async_trait]
impl WriteAckQuerier<MockChainContext, MockChainContext> for MockChainComponents {
    async fn query_write_ack_event(
        chain: &MockChainContext,
        packet: &PacketKey,
    ) -> Result<Option<WriteAckEvent>, Error> {
        let received = chain.get_received_packet_information(
            packet.dst_port_id.clone(),
            packet.dst_channel_id.clone(),
            packet.sequence,
        );

        if let Some((packet2, height)) = received {
            if &packet2 == packet {
                Ok(Some(WriteAckEvent::new(height)))
            } else {
                Err(BaseError::generic(eyre!(
                    "mismatch between packet in state {} and packet: {}",
                    packet2,
                    packet
                ))
                .into())
            }
        } else {
            Ok(None)
        }
    }
}

impl ProvideReceivePacketPayloadType<MockChainContext, MockChainContext> for MockChainComponents {
    type ReceivePacketPayload = MockMessage;
}

#[async_trait]
impl ReceivePacketPayloadBuilder<MockChainContext, MockChainContext> for MockChainComponents {
    async fn build_receive_packet_payload(
        chain: &MockChainContext,
        _client_state: &(),
        height: &MockHeight,
        packet: &PacketKey,
    ) -> Result<MockMessage, Error> {
        // If the latest state of the source chain doesn't have the packet as sent, return an error.
        let state = chain.get_current_state();
        if !state.check_sent((
            packet.src_port_id.clone(),
            packet.src_channel_id.clone(),
            packet.sequence,
        )) {
            return Err(BaseError::receive_without_sent(
                chain.name().to_string(),
                packet.src_channel_id.to_string(),
            )
            .into());
        }
        Ok(MockMessage::RecvPacket(*height, packet.clone()))
    }
}

#[async_trait]
impl ReceivePacketMessageBuilder<MockChainContext, MockChainContext> for MockChainComponents {
    async fn build_receive_packet_message(
        _chain: &MockChainContext,
        _packet: &PacketKey,
        payload: MockMessage,
    ) -> Result<MockMessage, Error> {
        Ok(payload)
    }
}

impl ProvideAckPacketPayloadType<MockChainContext, MockChainContext> for MockChainComponents {
    type AckPacketPayload = MockMessage;
}

#[async_trait]
impl AckPacketPayloadBuilder<MockChainContext, MockChainContext> for MockChainComponents {
    async fn build_ack_packet_payload(
        chain: &MockChainContext,
        _client_state: &(),
        height: &MockHeight,
        packet: &PacketKey,
        _ack: &WriteAckEvent,
    ) -> Result<MockMessage, Error> {
        // If the latest state of the destination chain doesn't have the packet as received, return an error.
        let state = chain.get_current_state();

        if !state.check_received((
            packet.dst_port_id.clone(),
            packet.dst_channel_id.clone(),
            packet.sequence,
        )) {
            return Err(BaseError::acknowledgment_without_received(
                chain.name().to_string(),
                packet.dst_channel_id.to_string(),
            )
            .into());
        }

        Ok(MockMessage::AckPacket(*height, packet.clone()))
    }
}

#[async_trait]
impl AckPacketMessageBuilder<MockChainContext, MockChainContext> for MockChainComponents {
    async fn build_ack_packet_message(
        _chain: &MockChainContext,
        _packet: &PacketKey,
        payload: MockMessage,
    ) -> Result<MockMessage, Error> {
        Ok(payload)
    }
}

impl ProvideTimeoutUnorderedPacketPayloadType<MockChainContext, MockChainContext>
    for MockChainComponents
{
    type TimeoutUnorderedPacketPayload = MockMessage;
}

#[async_trait]
impl TimeoutUnorderedPacketPayloadBuilder<MockChainContext, MockChainContext>
    for MockChainComponents
{
    async fn build_timeout_unordered_packet_payload(
        chain: &MockChainContext,
        _client_state: &(),
        height: &MockHeight,
        packet: &PacketKey,
    ) -> Result<MockMessage, Error> {
        let state = chain.get_current_state();
        let current_timestamp = chain.runtime.get_time();

        if !state.check_timeout(packet.clone(), *height, current_timestamp) {
            return Err(BaseError::timeout_without_sent(
                chain.name().to_string(),
                packet.src_channel_id.to_string(),
            )
            .into());
        }

        Ok(MockMessage::TimeoutPacket(*height, packet.clone()))
    }
}

#[async_trait]
impl TimeoutUnorderedPacketMessageBuilder<MockChainContext, MockChainContext>
    for MockChainComponents
{
    async fn build_timeout_unordered_packet_message(
        _chain: &MockChainContext,
        _packet: &PacketKey,
        payload: MockMessage,
    ) -> Result<MockMessage, Error> {
        Ok(payload)
    }
}
