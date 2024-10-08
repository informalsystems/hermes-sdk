use core::fmt::Display;

use cgp::prelude::*;

pub type DenomOf<Chain> = <Chain as HasDenomType>::Denom;

#[derive_component(DenomTypeComponent, ProvideDenomType<Chain>)]
pub trait HasDenomType: Async {
    type Denom: Display + Clone + Async;
}
