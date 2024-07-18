use cgp_core::prelude::*;

use crate::chain::types::aliases::ChainIdOf;
use crate::multi::types::index::Index;

#[derive_component(ChainTypeAtComponent, ProvideChainTypeAt<Context>)]
pub trait HasChainTypeAt<const I: usize>: Async {
    type Chain: HasErrorType;
}

pub trait HasChainAt<const I: usize>: HasChainTypeAt<I> {
    fn chain_at(&self, index: Index<I>) -> &Self::Chain;
}

pub type ChainTypeAt<Context, const I: usize> = <Context as HasChainTypeAt<I>>::Chain;

pub type ChainIdAt<Context, const I: usize> = ChainIdOf<ChainTypeAt<Context, I>>;
