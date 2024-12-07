use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;

#[cgp_component {
  name: BuilderTypeAtComponent,
  provider: ProvideBuilderTypeAt,
}]
pub trait HasBuilderTypeAt<A: Async, B: Async>: HasBiRelayTypeAt<A, B> {
    type Builder: Async;
}

pub type BuilderTypeAt<Context, A, B> = <Context as HasBuilderTypeAt<A, B>>::Builder;

impl<Context, A: Async, B: Async, Provider, Builder> ProvideBuilderTypeAt<Context, A, B>
    for WithProvider<Provider>
where
    Context: HasBiRelayTypeAt<A, B>,
    Provider: ProvideType<Context, BuilderTypeAtComponent, Type = Builder>,
    Builder: Async,
{
    type Builder = Builder;
}
