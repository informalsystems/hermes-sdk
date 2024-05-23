use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(CreateClientOptionsTypeComponent, ProvideCreateClientOptionsType<Chain>)]
pub trait HasCreateClientOptionsType<Counterparty>: Async {
    type CreateClientOptions: Async;
}

pub type CreateClientOptions<Chain, Counterparty> =
    <Chain as HasCreateClientOptionsType<Counterparty>>::CreateClientOptions;

#[derive_component(CreateClientPayloadTypeComponent, ProvideCreateClientPayloadType<Chain>)]
pub trait HasCreateClientPayloadType<Counterparty>: Async {
    type CreateClientPayload: Async;
}

#[derive_component(CreateClientEventComponent, ProvideCreateClientEvent<Chain>)]
pub trait HasCreateClientEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type CreateClientEvent: Async;

    fn try_extract_create_client_event(event: Self::Event) -> Option<Self::CreateClientEvent>;

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &Self::ClientId;
}
