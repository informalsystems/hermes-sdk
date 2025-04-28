use hermes_prelude::*;

use crate::traits::HasLightBlockType;

#[cgp_component {
  provider: VerificationStatusUpdater,
  context: Client,
}]
pub trait CanUpdateVerificationStatus<Status>: HasLightBlockType {
    fn update_verification_status(&mut self, _status: Status, block: &Self::LightBlock);
}

pub struct VerifiedStatus;

pub struct TrustedStatus;
