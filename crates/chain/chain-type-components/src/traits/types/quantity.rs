use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: QuantityTypeComponent,
  provider: ProvideQuantityType,
  context: Chain,
}]
pub trait HasQuantityType: Async {
    type Quantity: Display + Async;
}

pub type QuantityOf<Chain> = <Chain as HasQuantityType>::Quantity;

#[cgp_provider(QuantityTypeComponent)]
impl<Chain, Provider, Quantity> ProvideQuantityType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, QuantityTypeComponent, Type = Quantity>,
    Quantity: Display + Async,
{
    type Quantity = Quantity;
}
