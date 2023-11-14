use alloc::boxed::Box;
use cgp_core::prelude::*;

use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::amount::HasAmountType;
use crate::traits::chain::types::denom::HasDenomType;

#[derive_component(BalanceQuerierComponent, BalanceQuerier<Chain>)]
#[async_trait]
pub trait CanQueryBalance: HasAddressType + HasDenomType + HasAmountType + HasErrorType {
    async fn query_balance(
        &self,
        address: &Self::Address,
        denom: &Self::Denom,
    ) -> Result<Self::Amount, Self::Error>;
}
