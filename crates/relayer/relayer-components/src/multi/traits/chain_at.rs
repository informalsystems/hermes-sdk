use cgp_core::prelude::*;

use crate::multi::types::index::Index;

#[derive_component(ChainTypeAtComponent, ProvideChainTypeAt<Context>)]
pub trait HasChainTypeAt<const I: usize>: Async {
    type Chain: HasErrorType;
}

pub type ChainTypeAt<Context, const I: usize> = <Context as HasChainTypeAt<I>>::Chain;

pub trait HasChainAt<const I: usize>: HasChainTypeAt<I> {
    fn chain_at(&self, index: Index<I>) -> &Self::Chain;
}
