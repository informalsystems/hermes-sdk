use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_type]
pub trait HasBootstrapType<Tag>: Async {
    type Bootstrap: Async;
}

#[cgp_component {
    provider: BootstrapLoader,
    context: App,
}]
#[async_trait]
pub trait CanLoadBootstrap<Tag, Args: Async>: HasBootstrapType<Tag> + HasAsyncErrorType {
    async fn load_bootstrap(&self, args: &Args) -> Result<Self::Bootstrap, Self::Error>;
}
