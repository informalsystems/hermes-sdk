use hermes_prelude::*;

#[cgp_component {
  name: ChainNodeConfigTypeComponent,
  provider: ProvideChainNodeConfigType,
  context: Bootstrap,
}]
pub trait HasChainNodeConfigType: Async {
    type ChainNodeConfig: Async;
}
