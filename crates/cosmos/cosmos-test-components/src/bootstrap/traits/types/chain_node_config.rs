use cgp_core::prelude::*;

#[derive_component(ChainNodeConfigTypeComponent, ProvideChainNodeConfigType<Bootstrap>)]
pub trait HasChainNodeConfigType: Async {
    type ChainNodeConfig: Async;
}
