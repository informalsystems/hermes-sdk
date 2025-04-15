use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType, HasDenomType};

#[cgp_component {
  provider: BalanceQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryBalance:
    HasAddressType + HasDenomType + HasAmountType + HasAsyncErrorType
{
    async fn query_balance(
        &self,
        address: &Self::Address,
        denom: &Self::Denom,
    ) -> Result<Self::Amount, Self::Error>;
}
