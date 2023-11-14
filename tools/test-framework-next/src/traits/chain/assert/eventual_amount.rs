use cgp_core::prelude::*;

use crate::traits::chain::types::address::HasAddressType;
use crate::traits::chain::types::amount::HasAmountType;

#[async_trait]
pub trait CanAssertEventualAmount: HasAddressType + HasAmountType + HasErrorType {
    async fn assert_eventual_amount(
        &self,
        address: &Self::Address,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
