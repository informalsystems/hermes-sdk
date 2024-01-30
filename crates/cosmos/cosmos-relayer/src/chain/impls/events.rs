use alloc::sync::Arc;

use hermes_cosmos_client_components::methods::event::{
    try_extract_channel_open_init_event, try_extract_channel_open_try_event,
    try_extract_connection_open_init_event, try_extract_connection_open_try_event,
    try_extract_create_client_event, try_extract_send_packet_event, try_extract_write_ack_event,
};
use hermes_cosmos_client_components::types::events::channel::{
    CosmosChannelOpenInitEvent, CosmosChannelOpenTryEvent,
};
use hermes_cosmos_client_components::types::events::client::CosmosCreateClientEvent;
use hermes_cosmos_client_components::types::events::connection::{
    CosmosConnectionOpenInitEvent, CosmosConnectionOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    HasChannelOpenInitEvent, HasChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    HasConnectionOpenInitEvent, HasConnectionOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_types::core::ics04_channel::events::{SendPacket, WriteAcknowledgement};
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId};
use tendermint::abci::Event as AbciEvent;

use crate::contexts::chain::CosmosChain;

impl<Counterparty> HasCreateClientEvent<Counterparty> for CosmosChain {
    type CreateClientEvent = CosmosCreateClientEvent;

    fn try_extract_create_client_event(event: Arc<AbciEvent>) -> Option<CosmosCreateClientEvent> {
        try_extract_create_client_event(event)
    }

    fn create_client_event_client_id(event: &CosmosCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

impl<Counterparty> HasSendPacketEvent<Counterparty> for CosmosChain {
    type SendPacketEvent = SendPacket;

    fn try_extract_send_packet_event(event: &Arc<AbciEvent>) -> Option<SendPacket> {
        try_extract_send_packet_event(event)
    }

    fn extract_packet_from_send_packet_event(event: &SendPacket) -> Packet {
        event.packet.clone()
    }
}

impl<Counterparty> HasWriteAckEvent<Counterparty> for CosmosChain {
    type WriteAckEvent = WriteAcknowledgement;

    fn try_extract_write_ack_event(event: &Arc<AbciEvent>) -> Option<WriteAcknowledgement> {
        try_extract_write_ack_event(event)
    }
}

impl<Counterparty> HasConnectionOpenInitEvent<Counterparty> for CosmosChain {
    type ConnectionOpenInitEvent = CosmosConnectionOpenInitEvent;

    fn try_extract_connection_open_init_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosConnectionOpenInitEvent> {
        try_extract_connection_open_init_event(event)
    }

    fn connection_open_init_event_connection_id(
        event: &CosmosConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

impl<Counterparty> HasConnectionOpenTryEvent<Counterparty> for CosmosChain {
    type ConnectionOpenTryEvent = CosmosConnectionOpenTryEvent;

    fn try_extract_connection_open_try_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosConnectionOpenTryEvent> {
        try_extract_connection_open_try_event(event)
    }

    fn connection_open_try_event_connection_id(
        event: &CosmosConnectionOpenTryEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

impl<Counterparty> HasChannelOpenInitEvent<Counterparty> for CosmosChain {
    type ChannelOpenInitEvent = CosmosChannelOpenInitEvent;

    fn try_extract_channel_open_init_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosChannelOpenInitEvent> {
        try_extract_channel_open_init_event(event)
    }

    fn channel_open_init_event_channel_id(event: &CosmosChannelOpenInitEvent) -> &ChannelId {
        &event.channel_id
    }
}

impl<Counterparty> HasChannelOpenTryEvent<Counterparty> for CosmosChain {
    type ChannelOpenTryEvent = CosmosChannelOpenTryEvent;

    fn try_extract_channel_open_try_event(
        event: Arc<AbciEvent>,
    ) -> Option<CosmosChannelOpenTryEvent> {
        try_extract_channel_open_try_event(event)
    }

    fn channel_open_try_event_channel_id(event: &CosmosChannelOpenTryEvent) -> &ChannelId {
        &event.channel_id
    }
}
