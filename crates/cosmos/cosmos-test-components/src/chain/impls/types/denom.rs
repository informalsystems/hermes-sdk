use cgp_core::Async;
use hermes_test_components::chain::traits::types::denom::ProvideDenomType;

use crate::chain::types::denom::Denom;

pub struct ProvideIbcDenom;

impl<Chain> ProvideDenomType<Chain> for ProvideIbcDenom
where
    Chain: Async,
{
    type Denom = Denom;
}
