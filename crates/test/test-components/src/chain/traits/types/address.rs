use core::fmt::Display;

use cgp::prelude::*;

pub type AddressOf<ChainDriver> = <ChainDriver as HasAddressType>::Address;

#[derive_component(AddressTypeComponent, ProvideAddressType<Chain>)]
pub trait HasAddressType: Async {
    type Address: Display + Async;
}
