use cgp::prelude::*;

#[derive_component(IbcTransactionTypeComponent, ProvideIbcTransactionType<Chain>)]
pub trait HasIbcTransactionType<Counterparty>: Async {
    type IbcTransaction: Async;
}
