use hermes_cosmos_chain_components::types::events::client::CosmosCreateClientEvent;
use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientEvent;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    ProvideConnectionOpenInitEvent, ProvideConnectionOpenTryEvent,
};
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
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
