use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

#[cgp_component {
  provider: NextVerificationHeightComputer,
  context: Client,
}]
pub trait CanComputeNextVerificationHeight: HasHeightType + HasAsyncErrorType {
    fn compute_next_verification_height(
        &self,
        current_height: &Self::Height,
        target_height: &Self::Height,
    ) -> Result<Self::Height, Self::Error>;
}
