use core::fmt::Display;

use cgp_core::prelude::*;

#[derive_component(AddressTypeComponent, AddressTypeProvider<Chain>)]
pub trait HasAddressType: Async {
    type Address: Display + Async;
}
