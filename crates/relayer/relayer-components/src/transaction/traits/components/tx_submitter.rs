use cgp_core::prelude::*;

use crate::transaction::traits::types::{HasTransactionHashType, HasTransactionType};

#[derive_component(TxSubmitterComponent, TxSubmitter<TxContext>)]
#[async_trait]
pub trait CanSubmitTx: HasTransactionType + HasTransactionHashType + HasErrorType {
    async fn submit_tx(&self, tx: &Self::Transaction) -> Result<Self::TxHash, Self::Error>;
}
