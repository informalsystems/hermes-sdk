use cgp::prelude::*;

#[derive_component(ChainTypeComponent, ProvideChainType<Bootstrap>)]
pub trait HasChainType: Async {
    type Chain: Async;
}

#[derive_component(ChainGetterComponent, ChainGetter<Driver>)]
pub trait HasChain: HasChainType {
    fn chain(&self) -> &Self::Chain;
}
