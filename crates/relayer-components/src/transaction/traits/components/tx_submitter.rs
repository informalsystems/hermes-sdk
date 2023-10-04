use cgp_core::prelude::*;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;

#[derive_component(TxSubmitterComponent, TxSubmitter<TxContext>)]
#[async_trait]
pub trait CanSubmitTx: HasTxTypes {
    async fn submit_tx(&self, tx: &Self::Transaction) -> Result<Self::TxHash, Self::Error>;
}
