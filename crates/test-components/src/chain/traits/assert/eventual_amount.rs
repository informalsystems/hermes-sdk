use alloc::boxed::Box;
use cgp_core::prelude::*;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;

#[derive_component(EventualAmountAsserterComponent, EventualAmountAsserter<Chain>)]
#[async_trait]
pub trait CanAssertEventualAmount: HasAddressType + HasAmountType + HasErrorType {
    async fn assert_eventual_amount(
        &self,
        address: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
