use alloc::sync::Arc;
use core::marker::PhantomData;

use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_relayer_components::chain::traits::extract_data::EventExtractor;
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientEvent;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::{
    HasChannelIdType, HasClientIdType, HasConnectionIdType,
};
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    ProvideChannelOpenInitEvent, ProvideChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::ProvideSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::ProvideWriteAckEvent;
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

impl<Chain, Counterparty> ProvideCreateClientEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasClientIdType<Counterparty, ClientId = ClientId>
        + HasMessageResponseType<MessageResponse = Vec<Arc<AbciEvent>>>,
{
    type CreateClientEvent = CosmosCreateClientEvent;

    fn try_extract_create_client_event(
        events: &Vec<Arc<AbciEvent>>,
    ) -> Option<CosmosCreateClientEvent> {
        events.iter().find_map(|event| {
            if event.kind == "create_client" {
                for tag in &event.attributes {
                    if tag.key_bytes() == CLIENT_ID_ATTRIBUTE_KEY.as_bytes() {
                        let client_id = tag.value_str().ok()?.parse().ok()?;

                        return Some(CosmosCreateClientEvent { client_id });
                    }
                }

                None
            } else {
                None
            }
        })
    }

    fn create_client_event_client_id(event: &CosmosCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

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

impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for ProvideCosmosEvents
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasMessageResponseType<MessageResponse = Vec<Arc<AbciEvent>>>,
{
    type ConnectionOpenInitEvent = CosmosConnectionOpenInitEvent;

    fn try_extract_connection_open_init_event(
        events: &Vec<Arc<AbciEvent>>,
    ) -> Option<CosmosConnectionOpenInitEvent> {
        events.iter().find_map(|event| {
            let ibc_event = try_conn_open_init_from_abci_event(event).ok()??;
            let connection_id = ibc_event.conn_id_on_a().clone();

            Some(CosmosConnectionOpenInitEvent { connection_id })
        })
    }

    fn connection_open_init_event_connection_id(
        event: &CosmosConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

impl<Chain, Counterparty> ProvideConnectionOpenTryEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasConnectionIdType<Counterparty, ConnectionId = ConnectionId>
        + HasMessageResponseType<MessageResponse = Vec<Arc<AbciEvent>>>,
{
    type ConnectionOpenTryEvent = CosmosConnectionOpenTryEvent;

    fn try_extract_connection_open_try_event(
        _chain: &Chain,
        events: &Vec<Arc<AbciEvent>>,
    ) -> Option<CosmosConnectionOpenTryEvent> {
        events.iter().find_map(|event| {
            let ibc_event = try_conn_open_try_from_abci_event(event).ok()??;
            let connection_id = ibc_event.conn_id_on_b().clone();

            Some(CosmosConnectionOpenTryEvent { connection_id })
        })
    }

    fn connection_open_try_event_connection_id(
        event: &CosmosConnectionOpenTryEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

impl<Chain, Counterparty> ProvideChannelOpenInitEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasMessageResponseType<MessageResponse = Vec<Arc<AbciEvent>>>,
{
    type ChannelOpenInitEvent = CosmosChannelOpenInitEvent;

    fn try_extract_channel_open_init_event(
        events: &Vec<Arc<AbciEvent>>,
    ) -> Option<CosmosChannelOpenInitEvent> {
        events.iter().find_map(|event| {
            let ibc_event = try_chan_open_init_from_abci_event(event).ok()??;
            let channel_id = ibc_event.chan_id_on_a().clone();
            Some(CosmosChannelOpenInitEvent { channel_id })
        })
    }

    fn channel_open_init_event_channel_id(event: &CosmosChannelOpenInitEvent) -> &ChannelId {
        &event.channel_id
    }
}

impl<Chain, Counterparty> ProvideChannelOpenTryEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasChannelIdType<Counterparty, ChannelId = ChannelId>
        + HasMessageResponseType<MessageResponse = Vec<Arc<AbciEvent>>>,
{
    type ChannelOpenTryEvent = CosmosChannelOpenTryEvent;

    fn try_extract_channel_open_try_event(
        events: &Vec<Arc<AbciEvent>>,
    ) -> Option<CosmosChannelOpenTryEvent> {
        events.iter().find_map(|event| {
            let ibc_event = try_chan_open_try_from_abci_event(event).ok()??;
            let channel_id = ibc_event.chan_id_on_b().clone();
            Some(CosmosChannelOpenTryEvent { channel_id })
        })
    }

    fn channel_open_try_event_channel_id(event: &CosmosChannelOpenTryEvent) -> &ChannelId {
        &event.channel_id
    }
}

impl<Chain, Counterparty> ProvideSendPacketEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>
        + HasOutgoingPacketType<Counterparty, OutgoingPacket = Packet>,
{
    type SendPacketEvent = SendPacketEvent;

    fn try_extract_send_packet_event(event: &Arc<AbciEvent>) -> Option<SendPacketEvent> {
        try_send_packet_from_abci_event(event)
            .ok()?
            .map(|send_packet| send_packet.into())
    }

    fn extract_packet_from_send_packet_event(event: &SendPacketEvent) -> Packet {
        event.packet.clone()
    }
}

impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = WriteAckEvent;

    fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAckEvent> {
        try_write_acknowledgment_from_abci_event(event)
            .ok()?
            .map(|write_ack| write_ack.into())
    }

    fn write_acknowledgement(event: &WriteAckEvent) -> &Vec<u8> {
        &event.acknowledgment
    }
}
