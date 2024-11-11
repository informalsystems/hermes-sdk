use cgp::prelude::*;

use crate::transaction::traits::types::fee::HasFeeType;

#[derive_component(GasToFeeConverterComponent, GasToFeeConverter<TxContext>)]
#[async_trait]
pub trait CanConvertGasToFee: HasFeeType + HasErrorType {
    async fn gas_amount_to_fee(&self, gas: u64) -> Result<Self::Fee, Self::Error>;
}
