use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(AmountTypeComponent, ProvideAmountType<Chain>)]
pub trait HasAmountType: Async {
    type Amount: Display + Async;
}

pub type AmountOf<Chain> = <Chain as HasAmountType>::Amount;

impl<Chain, Provider, Amount> ProvideAmountType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, AmountTypeComponent, Type = Amount>,
    Amount: Display + Async,
{
    type Amount = Amount;
}
