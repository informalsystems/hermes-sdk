use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(UpdateClientPayloadTypeComponent, ProvideUpdateClientPayloadType<Chain>)]
pub trait HasUpdateClientPayloadType<Counterparty>: Async {
    type UpdateClientPayload: Async;
}

impl<Chain, Counterparty, Components, Delegate> ProvideUpdateClientPayloadType<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideUpdateClientPayloadType<Chain, Counterparty>,
{
    type UpdateClientPayload = Delegate::UpdateClientPayload;
}

impl<Chain, Counterparty, Provider, UpdateClientPayload>
    ProvideUpdateClientPayloadType<Chain, Counterparty> for WithProvider<Provider>
where
    Chain: Async,
    UpdateClientPayload: Async,
    Provider: ProvideType<Chain, UpdateClientPayloadTypeComponent, Type = UpdateClientPayload>,
{
    type UpdateClientPayload = UpdateClientPayload;
}
