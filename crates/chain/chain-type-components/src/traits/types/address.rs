use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(AddressTypeComponent, ProvideAddressType<Chain>)]
pub trait HasAddressType: Async {
    type Address: Display + Async;
}

pub type AddressOf<ChainDriver> = <ChainDriver as HasAddressType>::Address;

impl<Chain, Provider, Address> ProvideAddressType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, AddressTypeComponent, Type = Address>,
    Address: Display + Async,
{
    type Address = Address;
}
