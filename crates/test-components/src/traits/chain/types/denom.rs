use core::fmt::Display;

use cgp_core::Async;

pub trait HasDenomType: Async {
    type Denom: Display + Async;
}
