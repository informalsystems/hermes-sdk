use core::fmt::Display;

use cgp_core::prelude::*;

pub type Denom<Chain> = <Chain as HasDenomType>::Denom;

#[derive_component(DenomTypeComponent, DenomTypeProvider<Chain>)]
pub trait HasDenomType: Async {
    type Denom: Display + Clone + Async;
}
