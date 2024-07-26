use cgp_core::prelude::*;

#[derive_component(BootstrapTypeComponent, ProvideBootstrapType<App>)]
pub trait HasBootstrapType: Async {
    type Bootstrap: Async;
}

#[derive_component(BootstrapLoaderComponent, BootstrapLoader<App>)]
#[async_trait]
pub trait CanLoadBootstrap<Args: Async>: HasBootstrapType + HasErrorType {
    async fn load_bootstrap(&self, args: &Args) -> Result<Self::Bootstrap, Self::Error>;
}
