use cgp::prelude::*;
use hermes_chain_type_components::traits::HasHeightType;

use crate::traits::HasLightBlockType;

#[cgp_component {
  provider: LightBlockQuerier,
  context: Client,
}]
pub trait CanQueryLightBlock<Mode>: HasHeightType + HasLightBlockType {
    fn query_light_block(&self, _mode: Mode, height: &Self::Height) -> Option<Self::LightBlock>;
}

pub struct GetTrustedOrVerified;

pub struct GetHighestTrustedOrVerifiedBefore;

pub struct GetLowestTrustedOrVerified;
