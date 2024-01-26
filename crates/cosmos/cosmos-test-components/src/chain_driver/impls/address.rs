use cgp_core::Async;
use hermes_test_components::chain_driver::traits::types::address::AddressTypeProvider;

pub struct ProvideStringAddress;

impl<Chain> AddressTypeProvider<Chain> for ProvideStringAddress
where
    Chain: Async,
{
    type Address = String;
}
