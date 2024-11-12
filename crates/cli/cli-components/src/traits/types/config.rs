use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(ConfigTypeComponent, ProvideConfigType<App>)]
pub trait HasConfigType: Async {
    type Config: Async;
}

impl<App, Provider, Config> ProvideConfigType<App> for WithProvider<Provider>
where
    App: Async,
    Config: Async,
    Provider: ProvideType<ConfigTypeComponent, App, Type = Config>,
{
    type Config = Config;
}
