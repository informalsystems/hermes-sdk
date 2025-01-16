use cgp::prelude::*;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::denom::HasDenomType;

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
