use hermes_prelude::*;

#[cgp_type]
pub trait HasTxResponseType: Async {
    type TxResponse: Async;
}
