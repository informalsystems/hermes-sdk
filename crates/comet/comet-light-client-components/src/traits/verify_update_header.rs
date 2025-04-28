use hermes_prelude::*;

use crate::traits::{HasLightBlockType, HasVerdictType};

#[cgp_component {
  provider: UpdateHeaderVerifier,
  context: Client,
}]
pub trait CanVerifyUpdateHeader: HasLightBlockType + HasVerdictType + HasAsyncErrorType {
    fn verify_update_header(
        &self,
        untrusted_block: &Self::LightBlock,
        trusted_block: &Self::LightBlock,
    ) -> Result<Self::Verdict, Self::Error>;
}
