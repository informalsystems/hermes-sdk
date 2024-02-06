use cgp_core::prelude::*;

#[derive_component(RollupDriverTypeComponent, ProvideRollupDriverType<Bootstrap>)]
pub trait HasRollupDriverType: Async {
    type RollupDriver: Async;
}
