use core::fmt::Debug;

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: TimeTypeComponent,
  provider: ProvideTimeType,
  context: Chain,
}]
pub trait HasTimeType: Async {
    type Time: Async + Debug;
}

#[cgp_provider(TimeTypeComponent)]
impl<Chain, Provider, Time> ProvideTimeType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, TimeTypeComponent, Type = Time>,
    Time: Async + Debug,
{
    type Time = Time;
}
