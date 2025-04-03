use cgp::prelude::*;

#[cgp_type]
pub trait HasTxResponseType: Async {
    type TxResponse: Async;
}
