use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

#[derive_component(VerificationStatusUpdaterComponent, VerificationStatusUpdater<Chain>)]
pub trait CanUpdateVerificationStatus<Status>: HasHeightType {
    fn update_verification_status(&mut self, status: Status, height: &Self::Height);
}

pub struct VerifiedStatus;
