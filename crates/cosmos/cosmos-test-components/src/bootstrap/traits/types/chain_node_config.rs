use cgp::prelude::*;

#[cgp_component {
  name: ChainNodeConfigTypeComponent,
  provider: ProvideChainNodeConfigType,
  context: Bootstrap,
}]
pub trait HasChainNodeConfigType: Async {
    type ChainNodeConfig: Async;
}
