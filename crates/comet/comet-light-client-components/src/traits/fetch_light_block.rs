use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;
use crate::traits::types::status::HasVerificationStatusType;

#[cgp_component {
  provider: LightBlockFetcher,
  context: Client,
}]
#[async_trait]
pub trait CanFetchLightBlock: HasHeightType + HasLightBlockType + HasErrorType {
    async fn fetch_light_block(
        &self,
        height: &Self::Height,
    ) -> Result<Self::LightBlock, Self::Error>;
}

#[cgp_component {
  provider: LightBlockWithStatusFetcher,
  context: Client,
}]
#[async_trait]
pub trait CanFetchLightBlockWithStatus:
    HasHeightType + HasLightBlockType + HasVerificationStatusType + HasErrorType
{
    async fn fetch_light_block_with_status(
        &mut self,
        height: &Self::Height,
    ) -> Result<(Self::LightBlock, Self::VerificationStatus), Self::Error>;
}
