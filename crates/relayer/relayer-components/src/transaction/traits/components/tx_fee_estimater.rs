use cgp_core::prelude::*;

use crate::transaction::traits::types::HasTxTypes;

#[derive_component(TxFeeEstimatorComponent, TxFeeEstimator<TxContext>)]
#[async_trait]
pub trait CanEstimateTxFee: HasTxTypes {
    async fn estimate_tx_fee(&self, tx: &Self::Transaction) -> Result<Self::Fee, Self::Error>;
}
