use hermes_core::relayer_components::transaction::traits::HasFeeType;
use hermes_prelude::*;

#[cgp_component {
  provider: GasToFeeConverter,
  context: TxContext,
}]
#[async_trait]
pub trait CanConvertGasToFee: HasFeeType + HasAsyncErrorType {
    async fn gas_amount_to_fee(&self, gas: u64) -> Result<Self::Fee, Self::Error>;
}
