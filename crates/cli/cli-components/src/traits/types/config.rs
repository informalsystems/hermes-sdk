use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: ConfigTypeComponent,
  provider: ProvideConfigType,
  context: App,
}]
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
