use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;

#[async_trait]
pub trait CanBurnToken: HasAddressType + HasAmountType + HasErrorType {
    async fn burn_token(
        &self,
        sender: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
