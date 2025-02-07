use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: AmountTypeComponent,
  provider: ProvideAmountType,
  context: Chain,
}]
pub trait HasAmountType: Async {
    type Amount: Display + Async;
}

pub type AmountOf<Chain> = <Chain as HasAmountType>::Amount;

#[cgp_provider(AmountTypeComponent)]
impl<Chain, Provider, Amount> ProvideAmountType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, AmountTypeComponent, Type = Amount>,
    Amount: Display + Async,
{
    type Amount = Amount;
}
