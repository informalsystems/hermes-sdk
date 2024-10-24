use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[derive_component(ClientStateTypeComponent, ProvideClientStateType<Chain>)]
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
