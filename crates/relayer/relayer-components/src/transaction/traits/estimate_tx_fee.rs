use hermes_prelude::*;

use crate::transaction::traits::{HasFeeType, HasTransactionType};

#[cgp_component {
  provider: TxFeeEstimator,
  context: TxContext,
}]
#[async_trait]
pub trait CanEstimateTxFee: HasTransactionType + HasFeeType + HasAsyncErrorType {
    async fn estimate_tx_fee(&self, tx: &Self::Transaction) -> Result<Self::Fee, Self::Error>;
}
