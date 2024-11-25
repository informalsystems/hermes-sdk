use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

#[derive_component(LightBlockTracerComponent, LightBlockTracer<Chain>)]
pub trait CanTraceLightBlock: HasHeightType {
    fn trace_light_block(&mut self, target_height: &Self::Height, current_height: &Self::Height);
}
