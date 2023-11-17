use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

/// Generate an amount with denom, either deterministically or randomly,
/// for use on a bootstrapped chain. The generated amount should be sufficient
/// for use during testing.
#[async_trait]
pub trait CanGenerateAmount: HasAmountType {
    async fn generate_amount(&self) -> &Self::Amount;
}
