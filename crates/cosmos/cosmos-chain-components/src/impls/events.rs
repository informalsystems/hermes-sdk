use alloc::sync::Arc;

use hermes_relayer_components::chain::traits::types::create_client::ProvideCreateClientEvent;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics02_client::events::CLIENT_ID_ATTRIBUTE_KEY;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::events::IbcEventType;
use tendermint::abci::Event as AbciEvent;

use crate::types::events::client::CosmosCreateClientEvent;

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
