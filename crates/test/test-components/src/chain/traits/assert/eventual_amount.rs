use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasAddressType, HasAmountType};

#[cgp_component {
  provider: EventualAmountAsserter,
  context: Chain,
}]
#[async_trait]
pub trait CanAssertEventualAmount: HasAddressType + HasAmountType + HasAsyncErrorType {
    async fn assert_eventual_amount(
        &self,
        address: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
