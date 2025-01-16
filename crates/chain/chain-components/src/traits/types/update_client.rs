use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: UpdateClientPayloadTypeComponent,
  provider: ProvideUpdateClientPayloadType,
  context: Chain,
}]
pub trait HasUpdateClientPayloadType<Counterparty>: Async {
    type UpdateClientPayload: Async;
}

pub type UpdateClientPayloadOf<Chain, Counterparty> =
    <Chain as HasUpdateClientPayloadType<Counterparty>>::UpdateClientPayload;

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
