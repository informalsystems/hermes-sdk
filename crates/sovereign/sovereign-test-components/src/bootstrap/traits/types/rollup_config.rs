use cgp_core::prelude::*;

#[derive_component(RollupConfigTypeComponent, ProvideRollupConfigType<Bootstrap>)]
pub trait HasRollupConfigType: Async {
    type RollupConfig: Async;
}
