use hermes_cosmos_chain_components::types::events::client::CosmosCreateClientEvent;
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientEvent;
use hermes_relayer_components::chain::traits::types::event::HasEventType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    ProvideChannelOpenInitEvent, ProvideChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::ProvideSendPacketEvent;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::ProvideWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAcknowledgementType;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId};
use serde_json::Value;

use crate::types::event::SovereignEvent;

pub struct ProvideSovereignEvents;

impl<Chain, Counterparty> ProvideCreateClientEvent<Chain, Counterparty> for ProvideSovereignEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = SovereignEvent, ClientId = ClientId>,
{
    type CreateClientEvent = CosmosCreateClientEvent;

    fn try_extract_create_client_event(event: SovereignEvent) -> Option<CosmosCreateClientEvent> {
        if event.module_name != "ibc" {
            return None;
        }

        let create_client_event_json = event.event_value.get("CreateClient")?;

        let client_id_value = create_client_event_json
            .get("client_id")?
            .get("client_id")?;

        if let Value::String(client_id_str) = client_id_value {
            let client_id = client_id_str.parse().ok()?;
            Some(CosmosCreateClientEvent { client_id })
        } else {
            None
        }
    }

    fn create_client_event_client_id(event: &CosmosCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for ProvideSovereignEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = SovereignEvent, ConnectionId = ConnectionId>,
{
    type ConnectionOpenInitEvent = ConnectionId;

    fn try_extract_connection_open_init_event(event: SovereignEvent) -> Option<ConnectionId> {
        if event.module_name != "ibc" {
            return None;
        }

        let event_json = event.event_value.get("OpenInitConnection")?;

        let connection_id_value = event_json.get("connection_id")?;

        if let Value::String(connection_id_str) = connection_id_value {
            let connection_id = connection_id_str.parse().ok()?;
            Some(connection_id)
        } else {
            None
        }
    }

    fn connection_open_init_event_connection_id(connection_id: &ConnectionId) -> &ConnectionId {
        connection_id
    }
}

impl<Chain, Counterparty> ProvideConnectionOpenTryEvent<Chain, Counterparty>
    for ProvideSovereignEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = SovereignEvent, ConnectionId = ConnectionId>,
{
    type ConnectionOpenTryEvent = ConnectionId;

    fn try_extract_connection_open_try_event(event: SovereignEvent) -> Option<ConnectionId> {
        if event.module_name != "ibc" {
            return None;
        }

        let event_json = event.event_value.get("OpenTryConnection")?;

        let connection_id_value = event_json.get("connection_id")?;

        if let Value::String(connection_id_str) = connection_id_value {
            let connection_id = connection_id_str.parse().ok()?;
            Some(connection_id)
        } else {
            None
        }
    }

    fn connection_open_try_event_connection_id(connection_id: &ConnectionId) -> &ConnectionId {
        connection_id
    }
}

impl<Chain, Counterparty> ProvideChannelOpenInitEvent<Chain, Counterparty>
    for ProvideSovereignEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = SovereignEvent, ChannelId = ChannelId>,
{
    type ChannelOpenInitEvent = ChannelId;

    fn try_extract_channel_open_init_event(event: SovereignEvent) -> Option<ChannelId> {
        if event.module_name != "ibc" {
            return None;
        }

        let event_json = event.event_value.get("OpenInitChannel")?;

        let channel_id_value = event_json.get("chan_id_attr_on_a")?.get("channel_id")?;

        if let Value::String(channel_id_str) = channel_id_value {
            let channel_id = channel_id_str.parse().ok()?;
            Some(channel_id)
        } else {
            None
        }
    }

    fn channel_open_init_event_channel_id(channel_id: &ChannelId) -> &ChannelId {
        channel_id
    }
}

impl<Chain, Counterparty> ProvideChannelOpenTryEvent<Chain, Counterparty> for ProvideSovereignEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = SovereignEvent, ChannelId = ChannelId>,
{
    type ChannelOpenTryEvent = ChannelId;

    fn try_extract_channel_open_try_event(event: SovereignEvent) -> Option<ChannelId> {
        if event.module_name != "ibc" {
            return None;
        }

        let event_json = event.event_value.get("OpenTryChannel")?;

        let channel_id_value = event_json.get("chan_id_attr_on_b")?.get("channel_id")?;

        if let Value::String(channel_id_str) = channel_id_value {
            let channel_id = channel_id_str.parse().ok()?;
            Some(channel_id)
        } else {
            None
        }
    }

    fn channel_open_try_event_channel_id(channel_id: &ChannelId) -> &ChannelId {
        channel_id
    }
}

impl<Chain, Counterparty> ProvideSendPacketEvent<Chain, Counterparty> for ProvideSovereignEvents
where
    Chain: HasEventType<Event = SovereignEvent>
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>,
{
    type SendPacketEvent = Packet;

    fn try_extract_send_packet_event(event: &SovereignEvent) -> Option<Packet> {
        if event.module_name != "ibc" {
            return None;
        }

        println!("try extract send packet event: {:?}", event);

        // TODO
        None
    }

    fn extract_packet_from_send_packet_event(packet: &Packet) -> Packet {
        packet.clone()
    }
}

impl<Chain, Counterparty> ProvideWriteAckEvent<Chain, Counterparty> for ProvideSovereignEvents
where
    Chain: HasEventType<Event = SovereignEvent>
        + HasAcknowledgementType<Counterparty, Acknowledgement = Vec<u8>>,
{
    type WriteAckEvent = Vec<u8>;

    fn try_extract_write_ack_event(event: &SovereignEvent) -> Option<Vec<u8>> {
        if event.module_name != "ibc" {
            return None;
        }

        println!("try extract send packet event: {:?}", event);

        // TODO
        None
    }

    fn write_acknowledgement(ack: &Vec<u8>) -> &Vec<u8> {
        ack
    }
}
