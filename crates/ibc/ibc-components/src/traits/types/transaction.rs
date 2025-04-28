use hermes_prelude::*;

#[cgp_component {
  name: IbcTransactionTypeComponent,
  provider: ProvideIbcTransactionType,
  context: Chain,
}]
pub trait HasIbcTransactionType<Counterparty>: Async {
    type IbcTransaction: Async;
}
