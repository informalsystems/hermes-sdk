use cgp_async::async_generic_trait;
use cgp_macros::derive_component;

use crate::std_prelude::*;
use crate::transaction::traits::types::HasTxTypes;

#[derive_component(TxFeeEstimatorComponent, TxFeeEstimator<TxContext>)]
#[async_generic_trait]
pub trait CanEstimateTxFee: HasTxTypes {
    async fn estimate_tx_fee(&self, tx: &Self::Transaction) -> Result<Self::Fee, Self::Error>;
}
