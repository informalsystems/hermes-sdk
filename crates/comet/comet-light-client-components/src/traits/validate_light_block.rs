use hermes_prelude::*;

use crate::traits::HasLightBlockType;

#[cgp_component {
  provider: LightBlockValidator,
  context: Client,
}]
pub trait CanValidateLightBlock<Mode>: HasLightBlockType + HasAsyncErrorType {
    fn validate_light_block(
        &self,
        _mode: Mode,
        light_block: &Self::LightBlock,
    ) -> Result<(), Self::Error>;
}

pub struct IsWithinTrustingPeriod;
