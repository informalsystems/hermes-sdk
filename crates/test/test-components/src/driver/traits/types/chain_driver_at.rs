use cgp::prelude::*;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;

#[derive_component(ChainDriverTypeAtComponent, ProvideChainDriverTypeAt<Driver>)]
pub trait HasChainDriverTypeAt<const I: usize>: HasChainTypeAt<I> {
    type ChainDriver: Async;
}

pub type ChainDriverTypeAt<Driver, const I: usize> =
    <Driver as HasChainDriverTypeAt<I>>::ChainDriver;

#[derive_component(ChainDriverGetterAtComponent, ChainDriverGetterAt<Driver>)]
pub trait HasChainDriverAt<const I: usize>: HasChainDriverTypeAt<I> {
    fn chain_driver_at(&self, index: Index<I>) -> &Self::ChainDriver;
}
