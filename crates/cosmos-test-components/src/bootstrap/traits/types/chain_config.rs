use cgp_core::prelude::*;

#[derive_component(ChainConfigTypeComponent, ProvideChainConfigType<Bootstrap>)]
pub trait HasChainConfigType: Async {
    type ChainConfig: Async;
}
