use cgp_core::prelude::*;

#[derive_component(RollupGenesisConfigTypeComponent, ProvideRollupGenesisConfigType<Bootstrap>)]
pub trait HasRollupGenesisConfigType: Async {
    type RollupGenesisConfig: Async;
}
