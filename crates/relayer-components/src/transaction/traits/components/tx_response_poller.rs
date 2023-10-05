use cgp_core::prelude::*;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;

#[derive_component(TxResponsePollerComponent, TxResponsePoller<TxContext>)]
#[async_trait]
pub trait CanPollTxResponse: HasTxTypes {
    async fn poll_tx_response(
        &self,
        tx_hash: &Self::TxHash,
    ) -> Result<Self::TxResponse, Self::Error>;
}
