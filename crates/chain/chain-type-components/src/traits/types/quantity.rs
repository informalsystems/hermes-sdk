use core::fmt::Display;

use cgp::prelude::*;

#[derive_component(QuantityTypeComponent, ProvideQuantityType<Chain>)]
pub trait HasQuantityType: Async {
    type Quantity: Display + Async;
}

pub type QuantityOf<Chain> = <Chain as HasQuantityType>::Quantity;
