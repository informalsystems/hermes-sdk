use crate::driver::traits::types::chain::HasChainType;
use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::types::index::Index;
use cgp_core::prelude::*;

#[derive_component(ChainDriverTypeAtComponent, ProvideChainDriverTypeAt<Context>)]
pub trait HasChainDriverTypeAt<const I: usize>: HasChainTypeAt<I> {
    type ChainDriver: HasChainType<Chain = ChainTypeAt<Self, I>>;
}

pub type ChainDriverTypeAt<Context, const I: usize> =
    <Context as HasChainDriverTypeAt<I>>::ChainDriver;

pub trait HasChainDriverAt<const I: usize>: HasChainDriverTypeAt<I> {
    fn chain_driver_at(&self, index: Index<I>) -> &Self::ChainDriver;
}
