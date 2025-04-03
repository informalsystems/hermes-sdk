use cgp::prelude::*;

#[cgp_type]
pub trait HasTxHashType: Async {
    type TxHash: Async;
}
