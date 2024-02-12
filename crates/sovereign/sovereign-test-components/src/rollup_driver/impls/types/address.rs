use cgp_core::Async;
use hermes_test_components::chain_driver::traits::types::address::ProvideAddressType;

pub struct ProvideSovereignAddressType;

impl<RollupDriver> ProvideAddressType<RollupDriver> for ProvideSovereignAddressType
where
    RollupDriver: Async,
{
    type Address = String;
}
