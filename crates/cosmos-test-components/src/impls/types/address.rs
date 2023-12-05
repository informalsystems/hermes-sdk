use cgp_core::Async;
use ibc_test_components::chain::traits::types::address::AddressTypeProvider;

pub struct ProvideStringAddress;

impl<Chain> AddressTypeProvider<Chain> for ProvideStringAddress
where
    Chain: Async,
{
    type Address = String;
}
