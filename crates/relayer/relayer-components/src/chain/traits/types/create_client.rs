use cgp_core::Async;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub trait HasCreateClientOptions<Counterparty>: Async {
    type CreateClientPayloadOptions: Async;
}

pub trait HasCreateClientPayload<Counterparty>: Async {
    type CreateClientPayload: Async;
}

pub trait HasCreateClientEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type CreateClientEvent: Async;

    fn try_extract_create_client_event(event: Self::Event) -> Option<Self::CreateClientEvent>;

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &Self::ClientId;
}
