use cgp_core::prelude::*;

use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::denom::HasDenomType;
use crate::traits::chain::types::token::HasTokenType;

#[derive_component(BalanceQuerierComponent, BalanceQuerier<Chain>)]
#[async_trait]
pub trait CanQueryBalance: HasAddressType + HasDenomType + HasTokenType + HasErrorType {
    async fn query_balance(
        &self,
        wallet: &Self::Address,
        denom: &Self::Denom,
    ) -> Result<Self::Token, Self::Error>;
}
