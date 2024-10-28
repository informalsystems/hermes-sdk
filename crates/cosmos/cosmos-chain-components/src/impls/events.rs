use alloc::sync::Arc;

use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
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
use ibc_relayer::event::{
    channel_open_init_try_from_abci_event, channel_open_try_try_from_abci_event,
    connection_open_ack_try_from_abci_event, connection_open_try_try_from_abci_event,
    extract_packet_and_write_ack_from_tx,
};
use ibc_relayer_types::core::ics02_client::events::CLIENT_ID_ATTRIBUTE_KEY;
use ibc_relayer_types::core::ics04_channel::events::{SendPacket, WriteAcknowledgement};
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId};
use ibc_relayer_types::events::IbcEventType;
use tendermint::abci::Event as AbciEvent;

use crate::types::events::channel::{CosmosChannelOpenInitEvent, CosmosChannelOpenTryEvent};
use crate::types::events::client::CosmosCreateClientEvent;
use crate::types::events::connection::{
    CosmosConnectionOpenInitEvent, CosmosConnectionOpenTryEvent,
};

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
            let event_type = event.kind.parse().ok()?;

            if let IbcEventType::CreateClient = event_type {
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
            let event_type = event.kind.parse().ok()?;

            if let IbcEventType::OpenInitConnection = event_type {
                let open_ack_event = connection_open_ack_try_from_abci_event(event).ok()?;

                let connection_id = open_ack_event.connection_id()?.clone();

                Some(CosmosConnectionOpenInitEvent { connection_id })
            } else {
                None
            }
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
        events: &Vec<Arc<AbciEvent>>,
    ) -> Option<CosmosConnectionOpenTryEvent> {
        events.iter().find_map(|event| {
            let event_type = event.kind.parse().ok()?;

            if let IbcEventType::OpenTryConnection = event_type {
                let open_try_event = connection_open_try_try_from_abci_event(event).ok()?;

                let connection_id = open_try_event.connection_id()?.clone();

                Some(CosmosConnectionOpenTryEvent { connection_id })
            } else {
                None
            }
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
            let event_type = event.kind.parse().ok()?;

            if let IbcEventType::OpenInitChannel = event_type {
                let open_init_event = channel_open_init_try_from_abci_event(event).ok()?;

                let channel_id = open_init_event.channel_id()?.clone();

                Some(CosmosChannelOpenInitEvent { channel_id })
            } else {
                None
            }
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
            let event_type = event.kind.parse().ok()?;

            if let IbcEventType::OpenTryChannel = event_type {
                let open_try_event = channel_open_try_try_from_abci_event(event).ok()?;

                let channel_id = open_try_event.channel_id()?.clone();

                Some(CosmosChannelOpenTryEvent { channel_id })
            } else {
                None
            }
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
    type SendPacketEvent = SendPacket;

    fn try_extract_send_packet_event(event: &Arc<AbciEvent>) -> Option<SendPacket> {
        let event_type = event.kind.parse().ok()?;

        if let IbcEventType::SendPacket = event_type {
            let (packet, _) = extract_packet_and_write_ack_from_tx(event).ok()?;

            let send_packet_event = SendPacket { packet };

            Some(send_packet_event)
        } else {
            None
        }
    }

    fn extract_packet_from_send_packet_event(event: &SendPacket) -> Packet {
        event.packet.clone()
    }
}

impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasEventType<Event = Arc<AbciEvent>>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = WriteAcknowledgement;

    fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAcknowledgement> {
        if let IbcEventType::WriteAck = event.kind.parse().ok()? {
            let (packet, write_ack) = extract_packet_and_write_ack_from_tx(event).ok()?;

            let ack = WriteAcknowledgement {
                packet,
                ack: write_ack,
            };

            Some(ack)
        } else {
            None
        }
    }

    fn write_acknowledgement(event: &WriteAcknowledgement) -> &Vec<u8> {
        &event.ack
    }
}
