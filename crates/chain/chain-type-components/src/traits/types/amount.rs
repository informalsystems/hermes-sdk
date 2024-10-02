use core::fmt::Display;

use cgp::prelude::*;

#[derive_component(AmountTypeComponent, ProvideAmountType<Chain>)]
pub trait HasAmountType: Async {
    type Amount: Display + Async;
}

pub type AmountOf<Chain> = <Chain as HasAmountType>::Amount;
