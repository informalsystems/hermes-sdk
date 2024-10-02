use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;

#[async_trait]
pub trait CanMintToken: HasAddressType + HasAmountType + HasErrorType {
    async fn mint_token(
        &self,
        receiver: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
