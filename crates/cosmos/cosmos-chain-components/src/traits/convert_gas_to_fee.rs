use cgp::prelude::*;
use hermes_core::relayer_components::transaction::traits::HasFeeType;

#[cgp_component {
  provider: GasToFeeConverter,
  context: TxContext,
}]
#[async_trait]
pub trait CanConvertGasToFee: HasFeeType + HasAsyncErrorType {
    async fn gas_amount_to_fee(&self, gas: u64) -> Result<Self::Fee, Self::Error>;
}
