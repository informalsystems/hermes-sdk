use cgp_core::Async;
use hermes_test_components::chain_driver::traits::types::denom::ProvideDenomType;

pub struct ProvideSovereignDenomType;

impl<RollupDriver> ProvideDenomType<RollupDriver> for ProvideSovereignDenomType
where
    RollupDriver: Async,
{
    type Denom = String;
}
