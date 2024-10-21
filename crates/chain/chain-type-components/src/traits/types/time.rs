use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(TimeTypeComponent, ProvideTimeType<Chain>)]
pub trait HasTimeType: Async {
    type Time: Async;
}

impl<Chain, Provider, Time> ProvideTimeType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, TimeTypeComponent, Type = Time>,
    Time: Async,
{
    type Time = Time;
}
