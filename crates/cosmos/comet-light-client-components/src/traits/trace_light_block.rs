use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::state::HasVerifierStateType;

#[derive_component(LightBlockTracerComponent, LightBlockTracer<Chain>)]
pub trait CanTraceLightBlock: HasVerifierStateType + HasHeightType {
    fn trace_light_block(
        state: &mut Self::VerifierState,
        target_height: &Self::Height,
        current_height: &Self::Height,
    );
}
