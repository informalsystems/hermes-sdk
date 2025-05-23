use hermes_chain_type_components::traits::HasHeightType;
use hermes_prelude::*;

use crate::traits::HasLightBlockType;

#[cgp_component {
  provider: LightBlockHeightGetter,
  context: Client,
}]
pub trait HasLightBlockHeight: HasLightBlockType + HasHeightType {
    fn light_block_height(light_block: &Self::LightBlock) -> &Self::Height;
}
