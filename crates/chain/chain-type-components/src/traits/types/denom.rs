use core::fmt::Display;

use cgp::prelude::*;

#[derive_component(DenomTypeComponent, ProvideDenomType<Chain>)]
pub trait HasDenomType: Async {
    type Denom: Display + Async;
}

pub type DenomOf<Chain> = <Chain as HasDenomType>::Denom;
