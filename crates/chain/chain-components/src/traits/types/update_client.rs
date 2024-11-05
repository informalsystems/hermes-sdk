use cgp::core::component::UseDelegate;
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
