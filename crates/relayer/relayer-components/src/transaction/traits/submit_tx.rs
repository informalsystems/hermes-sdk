use cgp::prelude::*;

use crate::transaction::traits::types::transaction::HasTransactionType;
use crate::transaction::traits::types::tx_hash::HasTransactionHashType;

#[cgp_component {
  provider: TxSubmitter,
  context: TxContext,
}]
#[async_trait]
pub trait CanSubmitTx: HasTransactionType + HasTransactionHashType + HasErrorType {
    async fn submit_tx(&self, tx: &Self::Transaction) -> Result<Self::TxHash, Self::Error>;
}
