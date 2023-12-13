use cgp_core::Async;

pub trait HasChainTypeAt<const I: usize>: Async {
    type Chain: Async;
}

pub trait HasChainAt<const I: usize>: HasChainTypeAt<I> {
    fn chain(&self) -> &Self::Chain;
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
        self.chain()
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
        self.chain()
    }
}
