use cgp_core::Async;

pub trait HasTxContext: Async {
    type TxContext: Async;

    fn tx_context(&self) -> &Self::TxContext;
}
