use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;

#[cgp_component {
  provider: LightBlockHeightGetter,
  context: Client,
}]
pub trait HasLightBlockHeight: HasLightBlockType + HasHeightType {
    fn light_block_height(light_block: &Self::LightBlock) -> &Self::Height;
}
