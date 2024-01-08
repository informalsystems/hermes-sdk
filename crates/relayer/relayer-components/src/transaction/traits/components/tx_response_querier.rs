use cgp_core::prelude::*;

use crate::transaction::traits::types::HasTxTypes;

#[derive_component(TxResponseQuerierComponent, TxResponseQuerier<TxContext>)]
#[async_trait]
pub trait CanQueryTxResponse: HasTxTypes {
    async fn query_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Option<Self::TxResponse>, Self::Error>;
}
