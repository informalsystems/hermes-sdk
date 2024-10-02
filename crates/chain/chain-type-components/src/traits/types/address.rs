use core::fmt::Display;

use cgp::prelude::*;

#[derive_component(AddressTypeComponent, ProvideAddressType<Chain>)]
pub trait HasAddressType: Async {
    type Address: Display + Async;
}

pub type AddressOf<ChainDriver> = <ChainDriver as HasAddressType>::Address;
