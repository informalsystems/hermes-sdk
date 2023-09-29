use cgp_async::async_generic_trait;
use cgp_macros::derive_component;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;

#[derive_component(TxSubmitterComponent, TxSubmitter<TxContext>)]
#[async_generic_trait]
pub trait CanSubmitTx: HasTxTypes {
    async fn submit_tx(&self, tx: &Self::Transaction) -> Result<Self::TxHash, Self::Error>;
}
