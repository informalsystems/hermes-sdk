use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;
use crate::traits::types::state::HasVerifierStateType;
use crate::traits::types::status::HasVerificationStatusType;

#[derive_component(LightBlockFetcherComponent, LightBlockFetcher<Chain>)]
#[async_trait]
pub trait CanFetchLightBlock: HasHeightType + HasLightBlockType + HasErrorType {
    async fn fetch_light_block(
        &self,
        height: &Self::Height,
    ) -> Result<Self::LightBlock, Self::Error>;
}

#[derive_component(LightBlockWithStatusFetcherComponent, LightBlockWithStatusFetcher<Chain>)]
#[async_trait]
pub trait CanFetchLightBlockWithStatus:
    HasHeightType + HasVerifierStateType + HasLightBlockType + HasVerificationStatusType + HasErrorType
{
    async fn fetch_light_block_with_status(
        &self,
        height: &Self::Height,
        state: &mut Self::VerifierState,
    ) -> Result<(Self::LightBlock, Self::VerificationStatus), Self::Error>;
}
