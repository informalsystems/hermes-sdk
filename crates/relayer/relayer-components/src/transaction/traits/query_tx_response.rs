use cgp::prelude::*;

use crate::transaction::traits::{HasTxHashType, HasTxResponseType};

#[cgp_component {
  provider: TxResponseQuerier,
  context: TxContext,
}]
#[async_trait]
pub trait CanQueryTxResponse: HasTxHashType + HasTxResponseType + HasAsyncErrorType {
    async fn query_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Option<Self::TxResponse>, Self::Error>;
}
