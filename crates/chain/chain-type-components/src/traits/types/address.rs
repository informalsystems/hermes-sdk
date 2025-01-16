use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: AddressTypeComponent,
  provider: ProvideAddressType,
  context: Chain,
}]
pub trait HasAddressType: Async {
    type Address: Display + Async;
}

pub type AddressOf<Chain> = <Chain as HasAddressType>::Address;

impl<Chain, Provider, Address> ProvideAddressType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, AddressTypeComponent, Type = Address>,
    Address: Display + Async,
{
    type Address = Address;
}
