use cgp::prelude::*;
use hermes_relayer_components::transaction::traits::types::fee::HasFeeType;

#[cgp_component {
  provider: GasToFeeConverter,
  context: TxContext,
}]
#[async_trait]
pub trait CanConvertGasToFee: HasFeeType + HasErrorType {
    async fn gas_amount_to_fee(&self, gas: u64) -> Result<Self::Fee, Self::Error>;
}
