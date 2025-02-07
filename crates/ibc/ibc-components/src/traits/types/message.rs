use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
  name: IbcMessageTypeComponent,
  provider: ProvideIbcMessageType,
  context: Chain,
}]
pub trait HasIbcMessageType<Counterparty, App>: Async {
    type IbcMessage: Async;
}

#[cgp_provider(IbcMessageTypeComponent)]
impl<Chain, Counterparty, App, Components, Delegate> ProvideIbcMessageType<Chain, Counterparty, App>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<App, Delegate = Delegate>,
    Delegate: ProvideIbcMessageType<Chain, Counterparty, App>,
{
    type IbcMessage = Delegate::IbcMessage;
}
