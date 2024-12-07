use cgp::prelude::*;

#[cgp_component {
  name: TransactionHashTypeComponent,
  provider: ProvideTransactionHashType,
  context: Chain,
}]
pub trait HasTransactionHashType: Async {
    type TxHash: Async;
}
