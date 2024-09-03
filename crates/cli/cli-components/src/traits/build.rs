use cgp::prelude::*;

#[derive_component(BuilderTypeComponent, ProvideBuilderType<App>)]
pub trait HasBuilderType: Async {
    type Builder: Async;
}

pub type BuilderOf<Context> = <Context as HasBuilderType>::Builder;

#[derive_component(BuilderLoaderComponent, BuilderLoader<App>)]
#[async_trait]
pub trait CanLoadBuilder: HasBuilderType + HasErrorType {
    async fn load_builder(&self) -> Result<Self::Builder, Self::Error>;
}
