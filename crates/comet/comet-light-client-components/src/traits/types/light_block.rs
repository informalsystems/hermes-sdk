use cgp::prelude::*;

#[derive_component(LightBlockTypeComponent, ProvideLightBlockType<Client>)]
pub trait HasLightBlockType: Async {
    type LightBlock: Async;
}
