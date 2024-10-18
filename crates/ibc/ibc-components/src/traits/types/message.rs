use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[derive_component(IbcMessageTypeComponent, ProvideIbcMessageType<Chain>)]
pub trait HasIbcMessageType<Counterparty, App>: Async {
    type IbcMessage: Async;
}

impl<Chain, Counterparty, App, Components, Delegate> ProvideIbcMessageType<Chain, Counterparty, App>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<App, Delegate = Delegate>,
    Delegate: ProvideIbcMessageType<Chain, Counterparty, App>,
{
    type IbcMessage = Delegate::IbcMessage;
}
