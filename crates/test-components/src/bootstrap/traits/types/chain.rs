use cgp_core::prelude::*;

#[derive_component(ChainTypeComponent, ProvideChainType<Bootstrap>)]
pub trait HasChainType: Async {
    type Chain: Async;
}
