use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;
#[cgp_type {
    name: BuilderAtTypeProviderComponent<A, B>,
    provider: BuilderAtTypeProvider,
}]
pub trait HasBuilderTypeAt<A, B> {
    type Builder: Async;
}

pub type BuilderAt<Context, A, B> = <Context as HasBuilderTypeAt<A, B>>::Builder;
