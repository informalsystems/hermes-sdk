use cgp::prelude::*;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;
use crate::chain::traits::types::denom::HasDenomType;

#[derive_component(BalanceQuerierComponent, BalanceQuerier<Chain>)]
#[async_trait]
pub trait CanQueryBalance: HasAddressType + HasDenomType + HasAmountType + HasErrorType {
    async fn query_balance(
        &self,
        address: &Self::Address,
        denom: &Self::Denom,
    ) -> Result<Self::Amount, Self::Error>;
}
