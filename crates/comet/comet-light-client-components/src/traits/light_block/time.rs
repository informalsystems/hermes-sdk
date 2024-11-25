use cgp::prelude::*;
use hermes_chain_type_components::traits::types::time::HasTimeType;

use crate::traits::types::light_block::HasLightBlockType;

#[derive_component(LightBlockTimeGetterComponent, LightBlockHeightGetter<Client>)]
pub trait HasLightBlockTime: HasLightBlockType + HasTimeType {
    fn light_block_time(light_block: &Self::LightBlock) -> &Self::Time;
}
