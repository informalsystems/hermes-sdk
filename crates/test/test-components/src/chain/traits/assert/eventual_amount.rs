use cgp::prelude::*;

use crate::chain::traits::types::address::HasAddressType;
use crate::chain::traits::types::amount::HasAmountType;

#[cgp_component {
  name: EventualAmountAsserterComponent,
  provider: EventualAmountAsserter,
  context: Chain,
}]
#[async_trait]
pub trait CanAssertEventualAmount: HasAddressType + HasAmountType + HasErrorType {
    async fn assert_eventual_amount(
        &self,
        address: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
