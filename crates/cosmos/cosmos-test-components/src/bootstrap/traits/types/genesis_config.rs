use cgp::prelude::*;

#[cgp_component {
  name: ChainGenesisConfigTypeComponent,
  provider: ProvideChainGenesisConfigType,
  context: Bootstrap,
}]
pub trait HasChainGenesisConfigType: Async {
    type ChainGenesisConfig: Async;
}
