use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::light_block::HasLightBlockType;

#[derive_component(TargetHeightVerifierComponent, TargetHeightVerifier<Client>)]
#[async_trait]
pub trait CanVerifyTargetHeight<Mode: Async>:
    HasHeightType + HasLightBlockType + HasErrorType
{
    async fn verify_target_height(
        &mut self,
        _mode: Mode,
        target_height: &Self::Height,
    ) -> Result<Self::LightBlock, Self::Error>;
}

#[derive(Debug)]
pub struct NoInitialTrustedState;

pub struct VerifyToTarget;

pub struct VerifyForward;

pub struct VerifyBackward;