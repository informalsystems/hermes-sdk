use cgp::core::Async;
use hermes_test_components::chain::traits::types::address::ProvideAddressType;

pub struct ProvideStringAddress;

impl<Chain> ProvideAddressType<Chain> for ProvideStringAddress
where
    Chain: Async,
{
    type Address = String;
}
