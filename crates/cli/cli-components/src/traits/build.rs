use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: BuilderTypeComponent,
  provider: ProvideBuilderType,
  context: App,
}]
pub trait HasBuilderType: Async {
    type Builder: Async;
}

pub type BuilderOf<Context> = <Context as HasBuilderType>::Builder;

#[cgp_component {
  provider: BuilderLoader,
  context: App,
}]
#[async_trait]
pub trait CanLoadBuilder: HasBuilderType + HasErrorType {
    async fn load_builder(&self) -> Result<Self::Builder, Self::Error>;
}

impl<App, Provider, Builder> ProvideBuilderType<App> for WithProvider<Provider>
where
    App: Async,
    Builder: Async,
    Provider: ProvideType<BuilderTypeComponent, App, Type = Builder>,
{
    type Builder = Builder;
}
