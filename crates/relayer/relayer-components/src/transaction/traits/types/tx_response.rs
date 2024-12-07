use cgp::prelude::*;

#[cgp_component {
  name: TxResponseTypeComponent,
  provider: ProvideTxResponseType,
  context: Chain,
}]
pub trait HasTxResponseType: Async {
    type TxResponse: Async;
}
