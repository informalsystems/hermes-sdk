use cgp::prelude::*;

#[derive_component(IbcMessageHeaderTypeComponent, ProvideIbcMessageHeaderType<Chain>)]
pub trait HasIbcMessageHeaderType<Counterparty>: Async {
    type IbcMessageHeader: Async;
}
