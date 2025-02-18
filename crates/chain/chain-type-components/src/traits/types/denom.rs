use core::fmt::Display;

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: DenomTypeComponent,
  provider: ProvideDenomType,
  context: Chain,
}]
pub trait HasDenomType: Async {
    type Denom: Display + Async;
}

pub type DenomOf<Chain> = <Chain as HasDenomType>::Denom;

#[cgp_provider(DenomTypeComponent)]
impl<Chain, Provider, Denom> ProvideDenomType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, DenomTypeComponent, Type = Denom>,
    Denom: Display + Async,
{
    type Denom = Denom;
}
