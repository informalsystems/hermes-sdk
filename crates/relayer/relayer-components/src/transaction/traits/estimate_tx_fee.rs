use cgp::prelude::*;

use crate::transaction::traits::types::fee::HasFeeType;
use crate::transaction::traits::types::transaction::HasTransactionType;

#[cgp_component {
  name: TxFeeEstimatorComponent,
  provider: TxFeeEstimator,
  context: TxContext,
}]
#[async_trait]
pub trait CanEstimateTxFee: HasTransactionType + HasFeeType + HasErrorType {
    async fn estimate_tx_fee(&self, tx: &Self::Transaction) -> Result<Self::Fee, Self::Error>;
}
