use cgp::prelude::*;

#[cgp_component {
  name: TransactionTypeComponent,
  provider: ProvideTransactionType,
  context: Chain,
}]
pub trait HasTransactionType: Async {
    type Transaction: Async;

    fn tx_size(tx: &Self::Transaction) -> usize;
}
