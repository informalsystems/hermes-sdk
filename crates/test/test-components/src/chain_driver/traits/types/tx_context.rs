use cgp_core::prelude::*;

#[derive_component(TxContextTypeComponent, ProvideTxContextType<ChainDriver>)]
pub trait HasTxContextType: Async {
    type TxContext;
}

#[derive_component(TxContextGetterComponent, TxContextGetter<ChainDriver>)]
pub trait HasTxContext: HasTxContextType {
    fn tx_context(&self) -> &Self::TxContext;
}
