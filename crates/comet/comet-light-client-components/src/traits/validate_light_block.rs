use cgp::prelude::*;

use crate::traits::types::light_block::HasLightBlockType;

#[cgp_component {
  provider: LightBlockValidator,
  context: Client,
}]
pub trait CanValidateLightBlock<Mode>: HasLightBlockType + HasErrorType {
    fn validate_light_block(
        &self,
        _mode: Mode,
        light_block: &Self::LightBlock,
    ) -> Result<(), Self::Error>;
}

pub struct IsWithinTrustingPeriod;
