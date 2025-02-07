use cgp::prelude::*;
use hermes_test_components::chain::traits::types::address::{
    AddressTypeComponent, ProvideAddressType,
};

pub struct ProvideStringAddress;

#[cgp_provider(AddressTypeComponent)]
impl<Chain> ProvideAddressType<Chain> for ProvideStringAddress
where
    Chain: Async,
{
    type Address = String;
}
