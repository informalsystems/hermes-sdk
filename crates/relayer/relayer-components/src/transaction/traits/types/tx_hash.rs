use hermes_prelude::*;

#[cgp_type]
pub trait HasTxHashType: Async {
    type TxHash: Async;
}
