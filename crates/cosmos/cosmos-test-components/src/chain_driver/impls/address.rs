use cgp_core::Async;
use hermes_test_components::chain_driver::traits::types::address::ProvideAddressType;

pub struct ProvideStringAddress;

impl<Chain> ProvideAddressType<Chain> for ProvideStringAddress
where
    Chain: Async,
{
    type Address = String;
}
