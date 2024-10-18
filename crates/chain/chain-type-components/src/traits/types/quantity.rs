use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(QuantityTypeComponent, ProvideQuantityType<Chain>)]
pub trait HasQuantityType: Async {
    type Quantity: Display + Async;
}

pub type QuantityOf<Chain> = <Chain as HasQuantityType>::Quantity;

impl<Chain, Provider, Quantity> ProvideQuantityType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, QuantityTypeComponent, Type = Quantity>,
    Quantity: Display + Async,
{
    type Quantity = Quantity;
}
