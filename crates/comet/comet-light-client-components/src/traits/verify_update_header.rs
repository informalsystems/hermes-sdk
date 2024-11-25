use cgp::prelude::HasErrorType;

use crate::traits::types::light_block::HasLightBlockType;
use crate::traits::types::verdict::HasVerdictType;

pub trait CanVerifyUpdateHeader: HasLightBlockType + HasVerdictType + HasErrorType {
    fn verify_update_header(
        &self,
        untrusted_block: &Self::LightBlock,
        trusted_block: &Self::LightBlock,
    ) -> Result<Self::Verdict, Self::Error>;
}
