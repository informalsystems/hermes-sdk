use cgp_core::prelude::*;

#[derive_component(GenesisConfigTypeComponent, ProvideGenesisConfigType<Bootstrap>)]
pub trait HasGenesisConfigType: Async {
    type GenesisConfig: Async;
}
