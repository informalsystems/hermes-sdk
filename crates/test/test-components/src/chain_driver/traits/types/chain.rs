use cgp::prelude::*;

#[cgp_type]
pub trait HasChainType: Async {
    type Chain: Async;
}

#[cgp_getter {
    provider: ChainGetter,
}]
pub trait HasChain: HasChainType {
    fn chain(&self) -> &Self::Chain;
}
