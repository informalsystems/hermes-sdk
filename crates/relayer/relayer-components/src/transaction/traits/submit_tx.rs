use cgp::prelude::*;

use crate::transaction::traits::types::transaction::HasTransactionType;
use crate::transaction::traits::types::tx_hash::HasTxHashType;

#[cgp_component {
  provider: TxSubmitter,
  context: TxContext,
}]
#[async_trait]
pub trait CanSubmitTx: HasTransactionType + HasTxHashType + HasAsyncErrorType {
    async fn submit_tx(&self, tx: &Self::Transaction) -> Result<Self::TxHash, Self::Error>;
}
