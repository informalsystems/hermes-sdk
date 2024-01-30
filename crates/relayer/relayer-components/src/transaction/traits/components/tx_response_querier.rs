use cgp_core::prelude::*;

use crate::transaction::traits::types::{HasTransactionHashType, HasTxResponseType};

#[derive_component(TxResponseQuerierComponent, TxResponseQuerier<TxContext>)]
#[async_trait]
pub trait CanQueryTxResponse: HasTransactionHashType + HasTxResponseType + HasErrorType {
    async fn query_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Option<Self::TxResponse>, Self::Error>;
}
