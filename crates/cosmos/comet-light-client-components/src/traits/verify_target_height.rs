use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;
use crate::traits::types::state::HasVerifierStateType;

#[derive_component(TargetHeightVerifierComponent, TargetHeightVerifier<Chain>)]
#[async_trait]
pub trait CanVerifyTargetHeight<Mode: Async>:
    HasVerifierStateType + HasHeightType + HasLightBlockType + HasErrorType
{
    async fn verify_target_height(
        &self,
        _mode: Mode,
        state: &mut Self::VerifierState,
        target_height: &Self::Height,
    ) -> Result<Self::LightBlock, Self::Error>;
}

#[derive(Debug)]
pub struct NoInitialTrustedState;

pub struct VerifyToTarget;

pub struct VerifyForward;

pub struct VerifyBackward;
