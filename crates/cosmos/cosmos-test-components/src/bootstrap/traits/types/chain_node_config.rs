use cgp::prelude::*;

#[derive_component(ChainNodeConfigTypeComponent, ProvideChainNodeConfigType<Bootstrap>)]
pub trait HasChainNodeConfigType: Async {
    type ChainNodeConfig: Async;
}
