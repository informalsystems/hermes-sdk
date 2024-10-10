use cgp::prelude::*;

#[derive_component(IbcTransactionHeaderTypeComponent, ProvideIbcTransactionHeaderType<Chain>)]
pub trait HasIbcTransactionHeaderType<Counterparty>: Async {
    type IbcTransactionHeader: Async;
}
