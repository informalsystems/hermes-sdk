use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: BootstrapTypeComponent,
  provider: ProvideBootstrapType,
  context: App,
}]
pub trait HasBootstrapType: Async {
    type Bootstrap: Async;
}

#[cgp_component {
  provider: BootstrapLoader,
  context: App,
}]
#[async_trait]
pub trait CanLoadBootstrap<Args: Async>: HasBootstrapType + HasErrorType {
    async fn load_bootstrap(&self, args: &Args) -> Result<Self::Bootstrap, Self::Error>;
}

impl<App, Provider, Bootstrap> ProvideBootstrapType<App> for WithProvider<Provider>
where
    App: Async,
    Bootstrap: Async,
    Provider: ProvideType<BootstrapTypeComponent, App, Type = Bootstrap>,
{
    type Bootstrap = Bootstrap;
}
