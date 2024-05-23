use cgp_core::Async;
use hermes_test_components::chain::traits::types::denom::ProvideDenomType;

pub struct ProvideSovereignDenomType;

impl<Rollup> ProvideDenomType<Rollup> for ProvideSovereignDenomType
where
    Rollup: Async,
{
    type Denom = String;
}
