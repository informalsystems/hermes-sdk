use core::fmt::Display;

use cgp_core::prelude::*;

#[derive_component(DenomTypeComponent, DenomTypeProvider<Chain>)]
pub trait HasDenomType: Async {
    type Denom: Display + Clone + Async;
}
