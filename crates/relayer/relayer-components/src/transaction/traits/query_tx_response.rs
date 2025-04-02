use cgp::prelude::*;

use crate::transaction::traits::types::tx_hash::HasTxHashType;
use crate::transaction::traits::types::tx_response::HasTxResponseType;

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
