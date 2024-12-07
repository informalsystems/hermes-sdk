use cgp::prelude::*;

use crate::traits::types::light_block::HasLightBlockType;

#[cgp_component {
  name: VerificationStatusUpdaterComponent,
  provider: VerificationStatusUpdater,
  context: Client,
}]
pub trait CanUpdateVerificationStatus<Status>: HasLightBlockType {
    fn update_verification_status(&mut self, _status: Status, block: &Self::LightBlock);
}

pub struct VerifiedStatus;

pub struct TrustedStatus;
