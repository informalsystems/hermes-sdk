use cgp_core::prelude::*;

use crate::transaction::traits::types::fee::HasFeeType;
use crate::transaction::traits::types::transaction::HasTransactionType;

#[derive_component(TxFeeEstimatorComponent, TxFeeEstimator<TxContext>)]
#[async_trait]
pub trait CanEstimateTxFee: HasTransactionType + HasFeeType + HasErrorType {
    async fn estimate_tx_fee(&self, tx: &Self::Transaction) -> Result<Self::Fee, Self::Error>;
}
