use cgp::prelude::*;

#[derive_component(BiRelayTypeAtComponent, ProvideBiRelayTypeAt<Setup>)]
pub trait HasBiRelayTypeAt<const A: usize, const B: usize>: Async {
    type BiRelay: Async;
}

pub type BiRelayAt<Context, const A: usize, const B: usize> =
    <Context as HasBiRelayTypeAt<A, B>>::BiRelay;
