use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;

#[derive_component(BuilderTypeAtComponent, ProvideBuilderTypeAt<Context>)]
pub trait HasBuilderTypeAt<const A: usize, const B: usize>: HasBiRelayTypeAt<A, B> {
    type Builder: Async;
}

pub type BuilderTypeAt<Context, const A: usize, const B: usize> =
    <Context as HasBuilderTypeAt<A, B>>::Builder;

impl<Context, const A: usize, const B: usize, Provider, Builder> ProvideBuilderTypeAt<Context, A, B>
    for WithProvider<Provider>
where
    Context: HasBiRelayTypeAt<A, B>,
    Provider: ProvideType<Context, BuilderTypeAtComponent, Type = Builder>,
    Builder: Async,
{
    type Builder = Builder;
}
