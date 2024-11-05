use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

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
pub trait HasCreateClientEvent<Counterparty>:
    HasMessageResponseType + HasClientIdType<Counterparty>
{
    type CreateClientEvent: Async;

    fn try_extract_create_client_event(
        response: &Self::MessageResponse,
    ) -> Option<Self::CreateClientEvent>;

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &Self::ClientId;
}

impl<Chain, Counterparty, Components, Delegate>
    ProvideCreateClientMessageOptionsType<Chain, Counterparty> for UseDelegate<Components>
where
    Chain: Async,
    Delegate: ProvideCreateClientMessageOptionsType<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    type CreateClientMessageOptions = Delegate::CreateClientMessageOptions;
}

impl<Chain, Counterparty, Components, Delegate>
    ProvideCreateClientPayloadOptionsType<Chain, Counterparty> for UseDelegate<Components>
where
    Chain: Async,
    Delegate: ProvideCreateClientPayloadOptionsType<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    type CreateClientPayloadOptions = Delegate::CreateClientPayloadOptions;
}

impl<Chain, Counterparty, Components, Delegate> ProvideCreateClientPayloadType<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideCreateClientPayloadType<Chain, Counterparty>,
{
    type CreateClientPayload = Delegate::CreateClientPayload;
}

impl<Chain, Counterparty, Provider, CreateClientMessageOptions>
    ProvideCreateClientMessageOptionsType<Chain, Counterparty> for WithProvider<Provider>
where
    Chain: Async,
    CreateClientMessageOptions: Async,
    Provider:
        ProvideType<Chain, CreateClientPayloadTypeComponent, Type = CreateClientMessageOptions>,
{
    type CreateClientMessageOptions = CreateClientMessageOptions;
}

impl<Chain, Counterparty, Provider, CreateClientPayloadOptions>
    ProvideCreateClientPayloadOptionsType<Chain, Counterparty> for WithProvider<Provider>
where
    Chain: Async,
    CreateClientPayloadOptions: Async,
    Provider:
        ProvideType<Chain, CreateClientPayloadTypeComponent, Type = CreateClientPayloadOptions>,
{
    type CreateClientPayloadOptions = CreateClientPayloadOptions;
}

impl<Chain, Counterparty, Provider, CreateClientPayload>
    ProvideCreateClientPayloadType<Chain, Counterparty> for WithProvider<Provider>
where
    Chain: Async,
    CreateClientPayload: Async,
    Provider: ProvideType<Chain, CreateClientPayloadTypeComponent, Type = CreateClientPayload>,
{
    type CreateClientPayload = CreateClientPayload;
}
