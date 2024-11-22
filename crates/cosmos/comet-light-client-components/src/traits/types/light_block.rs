use cgp::prelude::*;

#[derive_component(LightBlockTypeComponent, ProvideLightBlockType<Chain>)]
pub trait HasLightBlockType: Async {
    type LightBlock: Async;
}
