use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

#[cgp_component {
  name: VerificationHeightTracerComponent,
  provider: VerificationHeightTracer,
  context: Client,
}]
pub trait CanTraceVerificationHeight: HasHeightType {
    fn trace_verification_height(
        &mut self,
        target_height: &Self::Height,
        current_height: &Self::Height,
    );
}
