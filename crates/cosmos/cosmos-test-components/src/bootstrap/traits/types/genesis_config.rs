use cgp::prelude::*;

#[derive_component(ChainGenesisConfigTypeComponent, ProvideChainGenesisConfigType<Bootstrap>)]
pub trait HasChainGenesisConfigType: Async {
    type ChainGenesisConfig: Async;
}
