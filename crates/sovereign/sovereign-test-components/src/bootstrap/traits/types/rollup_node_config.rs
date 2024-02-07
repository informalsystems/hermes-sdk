use cgp_core::prelude::*;

#[derive_component(RollupNodeConfigTypeComponent, ProvideRollupNodeConfigType<Bootstrap>)]
pub trait HasRollupNodeConfigType: Async {
    type RollupNodeConfig: Async;
}
