use hermes_prelude::*;

use crate::traits::{HasDivergenceType, HasLightBlockType};

#[cgp_component {
  provider: MisbehaviourDetector,
  context: Client,
}]
#[async_trait]
pub trait CanDetectMisbehaviour: HasLightBlockType + HasDivergenceType + HasAsyncErrorType {
    async fn detect(
        &self,
        target_block: &Self::LightBlock,
        trusted_block: &Self::LightBlock,
    ) -> Result<Option<Self::Divergence>, Self::Error>;
}
