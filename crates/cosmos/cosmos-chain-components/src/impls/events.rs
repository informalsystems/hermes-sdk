use alloc::sync::Arc;

use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientEvent;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::channel::{
    ProvideChannelOpenInitEvent, ProvideChannelOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenTryEvent,
};
use ibc_relayer::event::{
    channel_open_init_try_from_abci_event, channel_open_try_try_from_abci_event,
    connection_open_ack_try_from_abci_event, connection_open_try_try_from_abci_event,
};
use ibc_relayer_types::core::ics02_client::events::CLIENT_ID_ATTRIBUTE_KEY;
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
    Chain: HasIbcChainTypes<Counterparty, Event = Arc<AbciEvent>, ClientId = ClientId>,
{
    type CreateClientEvent = CosmosCreateClientEvent;

    fn try_extract_create_client_event(event: Arc<AbciEvent>) -> Option<CosmosCreateClientEvent> {
        let event_type = event.kind.parse().ok()?;

        if let IbcEventType::CreateClient = event_type {
            for tag in &event.attributes {
                let key = tag.key.as_str();
                let value = tag.value.as_str();
                if key == CLIENT_ID_ATTRIBUTE_KEY {
                    let client_id = value.parse().ok()?;

                    return Some(CosmosCreateClientEvent { client_id });
                }
            }

            None
        } else {
            None
        }
    }

    fn create_client_event_client_id(event: &CosmosCreateClientEvent) -> &ClientId {
        &event.client_id
    }
}

impl<Chain, Counterparty> ProvideConnectionOpenInitEvent<Chain, Counterparty>
    for ProvideCosmosEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = Arc<AbciEvent>, ConnectionId = ConnectionId>,
{
    type ConnectionOpenInitEvent = CosmosConnectionOpenInitEvent;

    fn try_extract_connection_open_init_event(
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

    fn connection_open_init_event_connection_id(
        event: &CosmosConnectionOpenInitEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

impl<Chain, Counterparty> ProvideConnectionOpenTryEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = Arc<AbciEvent>, ConnectionId = ConnectionId>,
{
    type ConnectionOpenTryEvent = CosmosConnectionOpenTryEvent;

    fn try_extract_connection_open_try_event(
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

    fn connection_open_try_event_connection_id(
        event: &CosmosConnectionOpenTryEvent,
    ) -> &ConnectionId {
        &event.connection_id
    }
}

impl<Chain, Counterparty> ProvideChannelOpenInitEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = Arc<AbciEvent>, ChannelId = ChannelId>,
{
    type ChannelOpenInitEvent = CosmosChannelOpenInitEvent;

    fn try_extract_channel_open_init_event(
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

    fn channel_open_init_event_channel_id(event: &CosmosChannelOpenInitEvent) -> &ChannelId {
        &event.channel_id
    }
}

impl<Chain, Counterparty> ProvideChannelOpenTryEvent<Chain, Counterparty> for ProvideCosmosEvents
where
    Chain: HasIbcChainTypes<Counterparty, Event = Arc<AbciEvent>, ChannelId = ChannelId>,
{
    type ChannelOpenTryEvent = CosmosChannelOpenTryEvent;

    fn try_extract_channel_open_try_event(
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

    fn channel_open_try_event_channel_id(event: &CosmosChannelOpenTryEvent) -> &ChannelId {
        &event.channel_id
    }
}
