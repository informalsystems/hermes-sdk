use hermes_core::chain_components::traits::{ProvideQuantityType, QuantityTypeComponent};
use hermes_prelude::*;

pub struct ProvideIbcQuantity;

#[cgp_provider(QuantityTypeComponent)]
impl<Chain> ProvideQuantityType<Chain> for ProvideIbcQuantity
where
    Chain: Async,
{
    type Quantity = u128;
}
