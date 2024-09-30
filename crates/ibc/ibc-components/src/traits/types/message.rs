use cgp::prelude::*;

#[derive_component(IbcMessageTypeComponent, ProvideIbcMessageType<Chain>)]
pub trait HasIbcMessageType<App, Counterparty>: Async {
    type IbcMessage;
}
