use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;
use hermes_relayer_components::multi::traits::birelay_at::HasBiRelayTypeAt;

#[cgp_type {
    provider: BuilderAtTypeProvider,
}]
pub trait HasBuilderTypeAt<A, B>: HasBiRelayTypeAt<A, B> {
    type Builder: Async;
}

pub type BuilderAt<Context, A, B> = <Context as HasBuilderTypeAt<A, B>>::Builder;
