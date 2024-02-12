use cgp_core::prelude::*;

#[derive_component(RollupTypeComponent, ProvideRollupType<Chain>)]
pub trait HasRollupType: Async {
    type Rollup: Async;
}

#[derive_component(RollupGetterComponent, RollupGetter<Chain>)]
pub trait HasRollup: HasRollupType {
    fn rollup(&self) -> &Self::Rollup;
}
