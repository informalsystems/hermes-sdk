use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(DenomTypeComponent, ProvideDenomType<Chain>)]
pub trait HasDenomType: Async {
    type Denom: Display + Async;
}

pub type DenomOf<Chain> = <Chain as HasDenomType>::Denom;

impl<Chain, Provider, Denom> ProvideDenomType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, DenomTypeComponent, Type = Denom>,
    Denom: Display + Async,
{
    type Denom = Denom;
}
