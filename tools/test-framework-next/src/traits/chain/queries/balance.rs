use cgp_core::prelude::*;

use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::amount::HasAmountType;

#[derive_component(BalanceQuerierComponent, BalanceQuerier<Chain>)]
#[async_trait]
pub trait CanQueryBalance: HasAddressType + HasAmountType + HasErrorType {
    async fn query_balance(&self, wallet: &Self::Address) -> Result<Self::Amount, Self::Error>;
}
