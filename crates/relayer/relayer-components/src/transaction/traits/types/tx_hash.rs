use cgp::prelude::*;

#[derive_component(TransactionHashTypeComponent, ProvideTransactionHashType<Chain>)]
pub trait HasTransactionHashType: Async {
    type TxHash: Async;
}
