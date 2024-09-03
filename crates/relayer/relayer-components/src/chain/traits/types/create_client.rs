use cgp::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(CreateClientPayloadOptionsTypeComponent, ProvideCreateClientPayloadOptionsType<Chain>)]
pub trait HasCreateClientPayloadOptionsType<Counterparty>: Async {
    type CreateClientPayloadOptions: Async;
}

pub type CreateClientPayloadOptionsOf<Chain, Counterparty> =
    <Chain as HasCreateClientPayloadOptionsType<Counterparty>>::CreateClientPayloadOptions;

#[derive_component(CreateClientMessageOptionsTypeComponent, ProvideCreateClientMessageOptionsType<Chain>)]
pub trait HasCreateClientMessageOptionsType<Counterparty>: Async {
    type CreateClientMessageOptions: Async;
}

pub type CreateClientMessageOptionsOf<Chain, Counterparty> =
    <Chain as HasCreateClientMessageOptionsType<Counterparty>>::CreateClientMessageOptions;

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
