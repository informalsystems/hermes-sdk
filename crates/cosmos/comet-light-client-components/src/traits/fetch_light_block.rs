use cgp::core::async_trait;
use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;

#[async_trait]
pub trait CanFetchLightBlock: HasHeightType + HasLightBlockType + HasErrorType {
    async fn fetch_light_block(
        &self,
        height: &Self::Height,
    ) -> Result<Self::LightBlock, Self::Error>;
}
