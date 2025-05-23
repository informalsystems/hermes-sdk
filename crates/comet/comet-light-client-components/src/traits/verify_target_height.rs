use hermes_chain_type_components::traits::HasHeightType;
use hermes_prelude::*;

use crate::traits::HasLightBlockType;

#[cgp_component {
  provider: TargetHeightVerifier,
  context: Client,
}]
#[async_trait]
pub trait CanVerifyTargetHeight<Mode: Async>:
    HasHeightType + HasLightBlockType + HasAsyncErrorType
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
