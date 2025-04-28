use hermes_chain_type_components::traits::HasHeightType;
use hermes_prelude::*;

#[cgp_component {
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
