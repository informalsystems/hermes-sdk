use alloc::sync::Arc;

use ibc_relayer::event::{
    channel_open_init_try_from_abci_event, channel_open_try_try_from_abci_event,
    connection_open_ack_try_from_abci_event, connection_open_try_try_from_abci_event,
    extract_packet_and_write_ack_from_tx,
};
use ibc_relayer_types::core::ics04_channel::events::{SendPacket, WriteAcknowledgement};
use ibc_relayer_types::events::IbcEventType;
use tendermint::abci::Event as AbciEvent;

use crate::types::events::channel::{CosmosChannelOpenInitEvent, CosmosChannelOpenTryEvent};
use crate::types::events::connection::{
    CosmosConnectionOpenInitEvent, CosmosConnectionOpenTryEvent,
};

pub fn try_extract_send_packet_event(event: &Arc<AbciEvent>) -> Option<SendPacket> {
    let event_type = event.kind.parse().ok()?;

    if let IbcEventType::SendPacket = event_type {
        let (packet, _) = extract_packet_and_write_ack_from_tx(event).ok()?;

        let send_packet_event = SendPacket { packet };

        Some(send_packet_event)
    } else {
        None
    }
}

pub fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAcknowledgement> {
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

pub fn try_extract_connection_open_init_event(
    event: Arc<AbciEvent>,
) -> Option<CosmosConnectionOpenInitEvent> {
    let event_type = event.kind.parse().ok()?;

    if let IbcEventType::OpenInitConnection = event_type {
        let open_ack_event = connection_open_ack_try_from_abci_event(&event).ok()?;

        let connection_id = open_ack_event.connection_id()?.clone();

        Some(CosmosConnectionOpenInitEvent { connection_id })
    } else {
        None
    }
}

pub fn try_extract_connection_open_try_event(
    event: Arc<AbciEvent>,
) -> Option<CosmosConnectionOpenTryEvent> {
    let event_type = event.kind.parse().ok()?;

    if let IbcEventType::OpenTryConnection = event_type {
        let open_try_event = connection_open_try_try_from_abci_event(&event).ok()?;

        let connection_id = open_try_event.connection_id()?.clone();

        Some(CosmosConnectionOpenTryEvent { connection_id })
    } else {
        None
    }
}

pub fn try_extract_channel_open_init_event(
    event: Arc<AbciEvent>,
) -> Option<CosmosChannelOpenInitEvent> {
    let event_type = event.kind.parse().ok()?;

    if let IbcEventType::OpenInitChannel = event_type {
        let open_init_event = channel_open_init_try_from_abci_event(&event).ok()?;

        let channel_id = open_init_event.channel_id()?.clone();

        Some(CosmosChannelOpenInitEvent { channel_id })
    } else {
        None
    }
}

pub fn try_extract_channel_open_try_event(
    event: Arc<AbciEvent>,
) -> Option<CosmosChannelOpenTryEvent> {
    let event_type = event.kind.parse().ok()?;

    if let IbcEventType::OpenTryChannel = event_type {
        let open_try_event = channel_open_try_try_from_abci_event(&event).ok()?;

        let channel_id = open_try_event.channel_id()?.clone();

        Some(CosmosChannelOpenTryEvent { channel_id })
    } else {
        None
    }
}
