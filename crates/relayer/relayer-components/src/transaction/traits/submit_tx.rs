use hermes_prelude::*;

use crate::transaction::traits::{HasTransactionType, HasTxHashType};

#[cgp_component {
  provider: TxSubmitter,
  context: TxContext,
}]
#[async_trait]
pub trait CanSubmitTx: HasTransactionType + HasTxHashType + HasAsyncErrorType {
    async fn submit_tx(&self, tx: &Self::Transaction) -> Result<Self::TxHash, Self::Error>;
}
