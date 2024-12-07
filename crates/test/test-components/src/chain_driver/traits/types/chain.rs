use cgp::prelude::*;

#[cgp_component {
  name: ChainTypeComponent,
  provider: ProvideChainType,
  context: Bootstrap,
}]
pub trait HasChainType: Async {
    type Chain: Async;
}

#[cgp_component {
  name: ChainGetterComponent,
  provider: ChainGetter,
  context: Driver,
}]
pub trait HasChain: HasChainType {
    fn chain(&self) -> &Self::Chain;
}
