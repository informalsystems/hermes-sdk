use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::state::HasVerifierStateType;

#[derive_component(VerificationStatusUpdaterComponent, VerificationStatusUpdater<Chain>)]
pub trait CanUpdateVerificationStatus<Status>: HasVerifierStateType + HasHeightType {
    fn update_verification_status(
        state: &mut Self::VerifierState,
        height: &Self::Height,
        status: Status,
    );
}

pub struct VerifiedStatus;
