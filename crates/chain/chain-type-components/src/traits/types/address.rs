use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_type]
pub trait HasAddressType: Async {
    type Address: Display + Async;
}
