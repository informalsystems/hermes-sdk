use cgp_core::prelude::*;

use crate::types::index::Index;

#[derive_component(ChainTypeAtComponent, ProvideChainTypeAt<Context>)]
pub trait HasChainTypeAt<const I: usize>: Async {
    type Chain: HasErrorType;
}

pub type ChainTypeAt<Context, const I: usize> = <Context as HasChainTypeAt<I>>::Chain;

pub trait HasChainAt<const I: usize>: HasChainTypeAt<I> {
    fn chain_at(&self, index: Index<I>) -> &Self::Chain;
}

/// Helper auto trait for accessing the first chain
pub trait HasOneChain: HasChainAt<0> {
    fn first_chain(&self) -> &Self::Chain;
}

impl<Context> HasOneChain for Context
where
    Context: HasChainAt<0>,
{
    fn first_chain(&self) -> &Self::Chain {
        self.chain_at(Index::<0>)
    }
}

/// Helper auto trait for accessing the second chain
pub trait HasTwoChains: HasChainAt<1> + HasOneChain {
    fn second_chain(&self) -> &<Self as HasChainTypeAt<1>>::Chain;
}

impl<Context> HasTwoChains for Context
where
    Context: HasChainAt<0> + HasChainAt<1>,
{
    fn second_chain(&self) -> &<Self as HasChainTypeAt<1>>::Chain {
        self.nth_chain::<1>()
    }
}

pub trait NthChain: Async {
    fn nth_chain<const I: usize>(&self) -> &Self::Chain
    where
        Self: HasChainAt<I>;
}

impl<Context> NthChain for Context
where
    Context: Async,
{
    fn nth_chain<const I: usize>(&self) -> &Context::Chain
    where
        Context: HasChainAt<I>,
    {
        self.chain_at(Index::<I>)
    }
}
