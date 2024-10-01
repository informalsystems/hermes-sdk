use cgp::prelude::*;

#[derive_component(IbcMessageTypeComponent, ProvideIbcMessageType<Chain>)]
pub trait HasIbcMessageType<Counterparty, App>: Async {
    type IbcMessage;
}
