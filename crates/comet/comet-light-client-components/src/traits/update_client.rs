use cgp::prelude::*;
use hermes_chain_type_components::traits::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;

#[cgp_component {
  provider: LightBlocksForUpdateClientBuilder,
  context: Client,
}]
#[async_trait]
pub trait CanBuildLightBlocksForUpdateClient:
    HasHeightType + HasLightBlockType + HasAsyncErrorType
{
    async fn build_light_blocks_for_update_client(
        &mut self,
        trusted_height: &Self::Height,
        target_height: &Self::Height,
    ) -> Result<Vec<Self::LightBlock>, Self::Error>;
}
