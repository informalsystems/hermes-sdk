use cgp::prelude::CanRaiseError;

use crate::traits::light_block::height::HasLightBlockHeight;
use crate::traits::state::query_light_block::{
    CanQueryLightBlock, GetHighestTrustedOrVerifiedBefore, GetLowestTrustedOrVerified,
    GetTrustedOrVerified,
};
use crate::traits::verify_target_height::{
    CanVerifyTargetHeight, NoInitialTrustedState, TargetHeightVerifier, VerifyBackward,
    VerifyForward, VerifyToTarget,
};

pub struct DoVerifyToTarget;

impl<Chain> TargetHeightVerifier<Chain, VerifyToTarget> for DoVerifyToTarget
where
    Chain: HasLightBlockHeight
        + CanVerifyTargetHeight<VerifyForward>
        + CanVerifyTargetHeight<VerifyBackward>
        + CanQueryLightBlock<GetTrustedOrVerified>
        + CanQueryLightBlock<GetHighestTrustedOrVerifiedBefore>
        + CanQueryLightBlock<GetLowestTrustedOrVerified>
        + CanRaiseError<NoInitialTrustedState>,
{
    async fn verify_target_height(
        chain: &Chain,
        _mode: VerifyToTarget,
        state: &mut Chain::VerifierState,
        target_height: &Chain::Height,
    ) -> Result<Chain::LightBlock, Chain::Error> {
        if let Some(block) = Chain::query_light_block(GetTrustedOrVerified, state, target_height) {
            return Ok(block);
        }

        let highest_block =
            Chain::query_light_block(GetHighestTrustedOrVerifiedBefore, state, target_height)
                .or_else(|| {
                    Chain::query_light_block(
                        GetHighestTrustedOrVerifiedBefore,
                        state,
                        target_height,
                    )
                })
                .ok_or_else(|| Chain::raise_error(NoInitialTrustedState))?;

        let highest_height = Chain::light_block_height(&highest_block);

        if target_height >= highest_height {
            chain
                .verify_target_height(VerifyForward, state, target_height)
                .await
        } else {
            chain
                .verify_target_height(VerifyBackward, state, target_height)
                .await
        }
    }
}
