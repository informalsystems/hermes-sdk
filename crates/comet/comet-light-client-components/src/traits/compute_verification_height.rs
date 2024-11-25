use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

#[derive_component(NextVerificationHeightComputerComponent, NextVerificationHeightComputer<Client>)]
pub trait CanComputeNextVerificationHeight: HasHeightType + HasErrorType {
    fn compute_next_verification_height(
        &self,
        current_height: &Self::Height,
        target_height: &Self::Height,
    ) -> Result<Self::Height, Self::Error>;
}
