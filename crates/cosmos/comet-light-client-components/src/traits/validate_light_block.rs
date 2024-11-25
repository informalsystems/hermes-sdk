use cgp::prelude::*;

use crate::traits::types::light_block::HasLightBlockType;

#[derive_component(LightBlockValidatorComponent, LightBlockValidator<Chain>)]
pub trait CanValidateLightBlock<Mode>: HasLightBlockType + HasErrorType {
    fn validate_light_block(
        &self,
        mode: Mode,
        light_block: &Self::LightBlock,
    ) -> Result<(), Self::Error>;
}

pub struct IsWithinTrustingPeriod;
