use cgp::prelude::*;

use crate::transaction::traits::types::tx_hash::HasTransactionHashType;
use crate::transaction::traits::types::tx_response::HasTxResponseType;

#[cgp_component {
  provider: TxResponsePoller,
  context: TxContext,
}]
#[async_trait]
pub trait CanPollTxResponse: HasTransactionHashType + HasTxResponseType + HasErrorType {
    async fn poll_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Self::TxResponse, Self::Error>;
}
