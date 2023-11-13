use core::fmt::Display;

use cgp_core::Async;

pub trait HasAddressType: Async {
    type Address: Display + Async;
}
