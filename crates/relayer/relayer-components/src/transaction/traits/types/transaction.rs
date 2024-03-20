use cgp_core::prelude::*;

#[derive_component(TransactionTypeComponent, ProvideTransactionType<Chain>)]
pub trait HasTransactionType: Async {
    type Transaction: Async;

    fn tx_size(tx: &Self::Transaction) -> usize;
}
