use cgp_core::prelude::HasErrorType;
use hermes_relayer_components::transaction::traits::estimate_tx_fee::TxFeeEstimator;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::types::fee::HasFeeType;
use hermes_relayer_components::transaction::traits::types::transaction::HasTransactionType;

pub struct ReturnSovereignTxFee<const FEE: u64>;

impl<Rollup, const FEE: u64> TxFeeEstimator<Rollup> for ReturnSovereignTxFee<FEE>
where
    Rollup: HasTransactionType + HasFeeType<Fee = u64> + HasErrorType,
{
    async fn estimate_tx_fee(
        _rollup: &Rollup,
        _tx: &Rollup::Transaction,
    ) -> Result<u64, Rollup::Error> {
        Ok(FEE)
    }
}

impl<Rollup, const FEE: u64> FeeForSimulationGetter<Rollup> for ReturnSovereignTxFee<FEE>
where
    Rollup: HasTransactionType + HasFeeType<Fee = u64> + HasErrorType,
{
    fn fee_for_simulation(_rollup: &Rollup) -> &u64 {
        &FEE
    }
}
