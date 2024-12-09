use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: ClientStateTypeComponent,
  provider: ProvideClientStateType,
  context: Chain,
}]
pub trait HasClientStateType<Counterparty>: Async {
    /**
        The client state of the `Self` chain's client on the `Counterparty` chain
    */
    type ClientState: Async;
}

impl<Chain, Counterparty, Components, Delegate> ProvideClientStateType<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideClientStateType<Chain, Counterparty>,
{
    type ClientState = Delegate::ClientState;
}

impl<Chain, Counterparty, Provider, ClientState> ProvideClientStateType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ClientStateTypeComponent, Type = ClientState>,
    ClientState: Async,
{
    type ClientState = ClientState;
}
