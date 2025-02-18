use alloc::sync::Arc;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_relayer_components::chain::traits::extract_data::{
    EventExtractor, EventExtractorComponent,
};
use hermes_relayer_components::chain::traits::packet::from_send_packet::{
    PacketFromSendPacketEventBuilder, PacketFromSendPacketEventBuilderComponent,
};
use hermes_relayer_components::chain::traits::packet::from_write_ack::{
    PacketFromWriteAckEventBuilder, PacketFromWriteAckEventBuilderComponent,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    CreateClientEventComponent, ProvideCreateClientEvent,
};
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::{
    HasChannelIdType, HasClientIdType, HasConnectionIdType,
};
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    ChannelOpenInitEventComponent, ChannelOpenTryEventComponent, ProvideChannelOpenInitEvent,
    ProvideChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ConnectionOpenInitEventComponent, ConnectionOpenTryEventComponent,
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::{
    HasSendPacketEvent, ProvideSendPacketEvent, SendPacketEventComponent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::{
    HasWriteAckEvent, ProvideWriteAckEvent, WriteAckEventComponent,
};
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAcknowledgementType;
use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::events::CLIENT_ID_ATTRIBUTE_KEY;
use ibc::core::host::types::identifiers::{ChannelId, ClientId, ConnectionId};
use tendermint::abci::Event as AbciEvent;

use crate::types::events::channel::{
    try_chan_open_init_from_abci_event, try_chan_open_try_from_abci_event,
    try_send_packet_from_abci_event, try_write_acknowledgment_from_abci_event,
    CosmosChannelOpenInitEvent, CosmosChannelOpenTryEvent,
};
use crate::types::events::client::CosmosCreateClientEvent;
use crate::types::events::connection::{
    try_conn_open_init_from_abci_event, try_conn_open_try_from_abci_event,
    CosmosConnectionOpenInitEvent, CosmosConnectionOpenTryEvent,
};
use crate::types::events::send_packet::SendPacketEvent;
use crate::types::events::write_acknowledgment::WriteAckEvent;

pub struct ProvideCosmosEvents;

#[cgp_provider(CreateClientEventComponent)]
impl<Chain, Counterparty> ProvideCreateClientEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>,
{
    type CreateClientEvent = CosmosCreateClientEvent;

    fn create_client_event_client_id(event: &CosmosCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, CosmosCreateClientEvent> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<CosmosCreateClientEvent>,
        event: &Chain::Event,
    ) -> Option<CosmosCreateClientEvent> {
        if event.kind == "create_client" {
            for tag in &event.attributes {
                if tag.key_bytes() == CLIENT_ID_ATTRIBUTE_KEY.as_bytes() {
                    let client_id = tag.value_str().ok()?.parse().ok()?;

                    return Some(CosmosCreateClientEvent { client_id });
                }
            }
        }

        None
    }
}

#[cgp_provider(ConnectionOpenInitEventComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for ProvideCosmosEvents
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>,
{
    type ConnectionOpenInitEvent = CosmosConnectionOpenInitEvent;

    fn connection_open_init_event_connection_id(
        event: &CosmosConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, CosmosConnectionOpenInitEvent> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<CosmosConnectionOpenInitEvent>,
        event: &Chain::Event,
    ) -> Option<CosmosConnectionOpenInitEvent> {
        let ibc_event = try_conn_open_init_from_abci_event(event).ok()??;
        let connection_id = ibc_event.conn_id_on_a().clone();

        Some(CosmosConnectionOpenInitEvent { connection_id })
    }
}

#[cgp_provider(ConnectionOpenTryEventComponent)]
impl<Chain, Counterparty> ProvideConnectionOpenTryEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>,
{
    type ConnectionOpenTryEvent = CosmosConnectionOpenTryEvent;

    fn connection_open_try_event_connection_id(
        event: &CosmosConnectionOpenTryEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, CosmosConnectionOpenTryEvent> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<CosmosConnectionOpenTryEvent>,
        event: &Chain::Event,
    ) -> Option<CosmosConnectionOpenTryEvent> {
        let ibc_event = try_conn_open_try_from_abci_event(event).ok()??;
        let connection_id = ibc_event.conn_id_on_b().clone();

        Some(CosmosConnectionOpenTryEvent { connection_id })
    }
}

#[cgp_provider(ChannelOpenInitEventComponent)]
impl<Chain, Counterparty> ProvideChannelOpenInitEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasMessageResponseType<MessageResponse = Vec<Arc<AbciEvent>>>,
{
    type ChannelOpenInitEvent = CosmosChannelOpenInitEvent;

    fn channel_open_init_event_channel_id(event: &CosmosChannelOpenInitEvent) -> &ChannelId {
        &event.channel_id
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, CosmosChannelOpenInitEvent> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<CosmosChannelOpenInitEvent>,
        event: &Chain::Event,
    ) -> Option<CosmosChannelOpenInitEvent> {
        let ibc_event = try_chan_open_init_from_abci_event(event).ok()??;
        let channel_id = ibc_event.chan_id_on_a().clone();
        Some(CosmosChannelOpenInitEvent { channel_id })
    }
}

#[cgp_provider(ChannelOpenTryEventComponent)]
impl<Chain, Counterparty> ProvideChannelOpenTryEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>,
{
    type ChannelOpenTryEvent = CosmosChannelOpenTryEvent;

    fn channel_open_try_event_channel_id(event: &CosmosChannelOpenTryEvent) -> &ChannelId {
        &event.channel_id
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, CosmosChannelOpenTryEvent> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<CosmosChannelOpenTryEvent>,
        event: &Chain::Event,
    ) -> Option<CosmosChannelOpenTryEvent> {
        let ibc_event = try_chan_open_try_from_abci_event(event).ok()??;
        let channel_id = ibc_event.chan_id_on_b().clone();
        Some(CosmosChannelOpenTryEvent { channel_id })
    }
}

#[cgp_provider(SendPacketEventComponent)]
impl<Chain, Counterparty> ProvideSendPacketEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>,
{
    type SendPacketEvent = SendPacketEvent;
}

#[cgp_provider(PacketFromSendPacketEventBuilderComponent)]
impl<Chain, Counterparty> PacketFromSendPacketEventBuilder<Chain, Counterparty>
    for ProvideCosmosEvents
where
    Chain: HasSendPacketEvent<Counterparty, SendPacketEvent = SendPacketEvent>
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>
        + HasAsyncErrorType,
{
    async fn build_packet_from_send_packet_event(
        _chain: &Chain,
        event: &SendPacketEvent,
    ) -> Result<Packet, Chain::Error> {
        Ok(event.packet.clone())
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, SendPacketEvent> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<SendPacketEvent>,
        event: &Chain::Event,
    ) -> Option<SendPacketEvent> {
        try_send_packet_from_abci_event(event)
            .ok()?
            .map(|send_packet| send_packet.into())
    }
}

#[cgp_provider(WriteAckEventComponent)]
impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = WriteAckEvent;
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain> EventExtractor<Chain, WriteAckEvent> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>,
{
    fn try_extract_from_event(
        _chain: &Chain,
        _tag: PhantomData<WriteAckEvent>,
        event: &Chain::Event,
    ) -> Option<WriteAckEvent> {
        try_write_acknowledgment_from_abci_event(event)
            .ok()?
            .map(|write_ack| write_ack.into())
    }
}

#[cgp_provider(PacketFromWriteAckEventBuilderComponent)]
impl<Chain, Counterparty> PacketFromWriteAckEventBuilder<Chain, Counterparty>
    for ProvideCosmosEvents
where
    Chain: HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAckEvent>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>
        + HasAsyncErrorType,
    Counterparty: HasOutgoingPacketType<Chain, OutgoingPacket = Packet>,
{
    async fn build_packet_from_write_ack_event(
        _chain: &Chain,
        ack: &WriteAckEvent,
    ) -> Result<Packet, Chain::Error> {
        Ok(ack.packet.clone())
    }

    async fn build_ack_from_write_ack_event(
        _chain: &Chain,
        event: &WriteAckEvent,
    ) -> Result<Vec<u8>, Chain::Error> {
        Ok(event.acknowledgment.clone())
    }
}
