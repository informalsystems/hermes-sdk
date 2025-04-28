use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::ProvideType;
use hermes_chain_type_components::traits::HasClientIdType;
use hermes_prelude::*;

#[cgp_component {
  name: CreateClientPayloadOptionsTypeComponent,
  provider: ProvideCreateClientPayloadOptionsType,
  context: Chain,
}]
pub trait HasCreateClientPayloadOptionsType<Counterparty>: Async {
    type CreateClientPayloadOptions: Async;
}

pub type CreateClientPayloadOptionsOf<Chain, Counterparty> =
    <Chain as HasCreateClientPayloadOptionsType<Counterparty>>::CreateClientPayloadOptions;

#[cgp_component {
  name: CreateClientMessageOptionsTypeComponent,
  provider: ProvideCreateClientMessageOptionsType,
  context: Chain,
}]
pub trait HasCreateClientMessageOptionsType<Counterparty>: Async {
    type CreateClientMessageOptions: Async;
}

pub type CreateClientMessageOptionsOf<Chain, Counterparty> =
    <Chain as HasCreateClientMessageOptionsType<Counterparty>>::CreateClientMessageOptions;

#[cgp_component {
  name: CreateClientPayloadTypeComponent,
  provider: ProvideCreateClientPayloadType,
  context: Chain,
}]
pub trait HasCreateClientPayloadType<Counterparty>: Async {
    type CreateClientPayload: Async;
}

pub type CreateClientPayloadOf<Chain, Counterparty> =
    <Chain as HasCreateClientPayloadType<Counterparty>>::CreateClientPayload;

#[cgp_component {
  name: CreateClientEventComponent,
  provider: ProvideCreateClientEvent,
  context: Chain,
}]
pub trait HasCreateClientEvent<Counterparty>: HasClientIdType<Counterparty> {
    type CreateClientEvent: Async;

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &Self::ClientId;
}

#[cgp_provider(CreateClientMessageOptionsTypeComponent)]
impl<Chain, Counterparty, Components, Delegate>
    ProvideCreateClientMessageOptionsType<Chain, Counterparty> for UseDelegate<Components>
where
    Chain: Async,
    Delegate: ProvideCreateClientMessageOptionsType<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    type CreateClientMessageOptions = Delegate::CreateClientMessageOptions;
}

#[cgp_provider(CreateClientPayloadOptionsTypeComponent)]
impl<Chain, Counterparty, Components, Delegate>
    ProvideCreateClientPayloadOptionsType<Chain, Counterparty> for UseDelegate<Components>
where
    Chain: Async,
    Delegate: ProvideCreateClientPayloadOptionsType<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    type CreateClientPayloadOptions = Delegate::CreateClientPayloadOptions;
}

#[cgp_provider(CreateClientPayloadTypeComponent)]
impl<Chain, Counterparty, Components, Delegate> ProvideCreateClientPayloadType<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideCreateClientPayloadType<Chain, Counterparty>,
{
    type CreateClientPayload = Delegate::CreateClientPayload;
}

#[cgp_provider(CreateClientMessageOptionsTypeComponent)]
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

#[cgp_provider(CreateClientPayloadOptionsTypeComponent)]
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

#[cgp_provider(CreateClientPayloadTypeComponent)]
impl<Chain, Counterparty, Provider, CreateClientPayload>
    ProvideCreateClientPayloadType<Chain, Counterparty> for WithProvider<Provider>
where
    Chain: Async,
    CreateClientPayload: Async,
    Provider: ProvideType<Chain, CreateClientPayloadTypeComponent, Type = CreateClientPayload>,
{
    type CreateClientPayload = CreateClientPayload;
}
