use cgp::prelude::*;
use hermes_chain_type_components::traits::HasHeightType;

use crate::traits::{HasLightBlockType, HasVerificationStatusType};

#[cgp_component {
  provider: LightBlockFetcher,
  context: Client,
}]
#[async_trait]
pub trait CanFetchLightBlock: HasHeightType + HasLightBlockType + HasAsyncErrorType {
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
    HasHeightType + HasLightBlockType + HasVerificationStatusType + HasAsyncErrorType
{
    async fn fetch_light_block_with_status(
        &mut self,
        height: &Self::Height,
    ) -> Result<(Self::LightBlock, Self::VerificationStatus), Self::Error>;
}
