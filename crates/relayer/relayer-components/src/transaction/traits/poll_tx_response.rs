use cgp::prelude::*;

use crate::transaction::traits::{HasTxHashType, HasTxResponseType};

#[cgp_component {
  provider: TxResponsePoller,
  context: TxContext,
}]
#[async_trait]
pub trait CanPollTxResponse: HasTxHashType + HasTxResponseType + HasAsyncErrorType {
    async fn poll_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Self::TxResponse, Self::Error>;
}
