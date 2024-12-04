use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;
use crate::traits::types::status::HasVerificationStatusType;

#[derive_component(LightBlockFetcherComponent, LightBlockFetcher<Client>)]
#[async_trait]
pub trait CanFetchLightBlock: HasHeightType + HasLightBlockType + HasErrorType {
    async fn fetch_light_block(
        &self,
        height: &Self::Height,
    ) -> Result<Self::LightBlock, Self::Error>;
}

#[derive_component(LightBlockWithStatusFetcherComponent, LightBlockWithStatusFetcher<Client>)]
#[async_trait]
pub trait CanFetchLightBlockWithStatus:
    HasHeightType + HasLightBlockType + HasVerificationStatusType + HasErrorType
{
    async fn fetch_light_block_with_status(
        &mut self,
        height: &Self::Height,
    ) -> Result<(Self::LightBlock, Self::VerificationStatus), Self::Error>;
}
