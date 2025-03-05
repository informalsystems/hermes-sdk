use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_type]
pub trait HasBootstrapType<Tag> {
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

#[cgp_provider(BootstrapTypeProviderComponent)]
impl<Context, Tag, Components, Delegate> BootstrapTypeProvider<Context, Tag>
    for UseDelegate<Components>
where
    Components: DelegateComponent<Tag, Delegate = Delegate>,
    Delegate: BootstrapTypeProvider<Context, Tag>,
{
    type Bootstrap = Delegate::Bootstrap;
}
