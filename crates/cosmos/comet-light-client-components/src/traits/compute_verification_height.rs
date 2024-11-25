use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::state::HasVerifierStateType;

pub trait CanComputeNextVerificationHeight: HasVerifierStateType + HasHeightType {
    fn compute_next_verification_height(
        state: &Self::VerifierState,
        current_height: &Self::Height,
        target_height: &Self::Height,
    ) -> Self::Height;
}
