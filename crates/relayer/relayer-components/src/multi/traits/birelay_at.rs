use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(BiRelayTypeAtComponent, ProvideBiRelayTypeAt<Setup>)]
pub trait HasBiRelayTypeAt<const A: usize, const B: usize>: Async {
    type BiRelay: Async;
}

pub type BiRelayAt<Context, const A: usize, const B: usize> =
    <Context as HasBiRelayTypeAt<A, B>>::BiRelay;

impl<Context, const A: usize, const B: usize, Provider, BiRelay> ProvideBiRelayTypeAt<Context, A, B>
    for WithProvider<Provider>
where
    Context: Async,
    Provider: ProvideType<Context, BiRelayTypeAtComponent, Type = BiRelay>,
    BiRelay: Async,
{
    type BiRelay = BiRelay;
}
