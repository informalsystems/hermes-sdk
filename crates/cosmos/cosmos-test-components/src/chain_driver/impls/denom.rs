use cgp_core::Async;
use hermes_test_components::chain_driver::traits::types::denom::ProvideDenomType;

use crate::chain_driver::types::denom::Denom;

pub struct ProvideIbcDenom;

impl<Chain> ProvideDenomType<Chain> for ProvideIbcDenom
where
    Chain: Async,
{
    type Denom = Denom;
}
