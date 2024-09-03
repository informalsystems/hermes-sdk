use cgp::prelude::*;

#[derive_component(TxResponseTypeComponent, ProvideTxResponseType<Chain>)]
pub trait HasTxResponseType: Async {
    type TxResponse: Async;
}
