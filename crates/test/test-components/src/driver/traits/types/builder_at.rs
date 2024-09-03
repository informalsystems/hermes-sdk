use cgp::prelude::*;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;

#[derive_component(BuilderTypeAtComponent, ProvideBuilderTypeAt<Context>)]
pub trait HasBuilderTypeAt<const A: usize, const B: usize>: HasBiRelayTypeAt<A, B> {
    type Builder: HasBiRelayTypeAt<0, 1, BiRelay = Self::BiRelay>;
}

pub type BuilderTypeAt<Context, const A: usize, const B: usize> =
    <Context as HasBuilderTypeAt<A, B>>::Builder;
