use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;
use crate::traits::types::state::HasVerifierStateType;

#[derive_component(TrustedOrVerifiedQuerierComponent, TrustedOrVerifiedQuerier<Chain>)]
pub trait CanQueryLightBlock<Mode>:
    HasVerifierStateType + HasHeightType + HasLightBlockType
{
    fn query_light_block(
        _mode: Mode,
        state: &Self::VerifierState,
        height: &Self::Height,
    ) -> Option<Self::LightBlock>;
}

pub struct GetTrustedOrVerified;

pub struct GetHighestTrustedOrVerifiedBefore;

pub struct GetLowestTrustedOrVerified;
