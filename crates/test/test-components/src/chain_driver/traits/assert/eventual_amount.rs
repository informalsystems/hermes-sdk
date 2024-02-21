use cgp_core::prelude::*;

use crate::chain::traits::types::address::{AddressOf, HasAddressType};
use crate::chain::traits::types::amount::{AmountOf, HasAmountType};
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive_component(EventualAmountAsserterComponent, EventualAmountAsserter<Chain>)]
#[async_trait]
pub trait CanAssertEventualAmount: HasChainType + HasErrorType
where
    Self::Chain: HasAddressType + HasAmountType,
{
    async fn assert_eventual_amount(
        &self,
        address: &AddressOf<Self::Chain>,
        amount: &AmountOf<Self::Chain>,
    ) -> Result<(), Self::Error>;
}
