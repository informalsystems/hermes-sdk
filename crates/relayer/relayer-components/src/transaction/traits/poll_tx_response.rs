use cgp_core::prelude::*;

use crate::transaction::traits::types::tx_hash::HasTransactionHashType;
use crate::transaction::traits::types::tx_response::HasTxResponseType;

#[derive_component(TxResponsePollerComponent, TxResponsePoller<TxContext>)]
#[async_trait]
pub trait CanPollTxResponse: HasTransactionHashType + HasTxResponseType + HasErrorType {
    async fn poll_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Self::TxResponse, Self::Error>;
}
