use hermes_chain_type_components::traits::types::height::HasHeightType;

pub trait CanComputeNextVerificationHeight: HasHeightType {
    fn compute_next_verification_height(
        &self,
        current_height: &Self::Height,
        target_height: &Self::Height,
    ) -> Self::Height;
}
