use core::marker::PhantomData;

use cgp_core::HasErrorType;

use crate::transaction::traits::components::tx_fee_estimater::TxFeeEstimator;
use crate::transaction::traits::types::fee::HasFeeType;
use crate::transaction::traits::types::transaction::HasTransactionType;

pub trait CanRecoverEstimateError: HasFeeType + HasErrorType {
    fn try_recover_estimate_error(&self, e: Self::Error) -> Result<Self::Fee, Self::Error>;
}

pub struct TryRecoverEstimateError<InEstimator>(pub PhantomData<InEstimator>);

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
