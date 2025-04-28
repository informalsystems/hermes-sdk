use core::marker::PhantomData;

use hermes_prelude::*;

use crate::transaction::traits::{
    HasFeeType, HasTransactionType, TxFeeEstimator, TxFeeEstimatorComponent,
};

pub trait CanRecoverEstimateError: HasFeeType + HasAsyncErrorType {
    fn try_recover_estimate_error(&self, e: Self::Error) -> Result<Self::Fee, Self::Error>;
}

pub struct TryRecoverEstimateError<InEstimator>(pub PhantomData<InEstimator>);

#[cgp_provider(TxFeeEstimatorComponent)]
impl<Context, InEstimator> TxFeeEstimator<Context> for TryRecoverEstimateError<InEstimator>
where
    Context: CanRecoverEstimateError + HasTransactionType,
    InEstimator: TxFeeEstimator<Context>,
{
    async fn estimate_tx_fee(
        context: &Context,
        tx: &Context::Transaction,
    ) -> Result<Context::Fee, Context::Error> {
        let res = InEstimator::estimate_tx_fee(context, tx).await;

        match res {
            Ok(fee) => Ok(fee),
            Err(e) => context.try_recover_estimate_error(e),
        }
    }
}
