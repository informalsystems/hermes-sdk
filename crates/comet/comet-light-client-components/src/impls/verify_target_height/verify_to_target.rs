use cgp::prelude::CanRaiseError;

use crate::traits::light_block::height::HasLightBlockHeight;
use crate::traits::query_light_block::{
    CanQueryLightBlock, GetHighestTrustedOrVerifiedBefore, GetLowestTrustedOrVerified,
    GetTrustedOrVerified,
};
use crate::traits::verify_target_height::{
    CanVerifyTargetHeight, NoInitialTrustedState, TargetHeightVerifier, VerifyBackward,
    VerifyForward, VerifyToTarget,
};

pub struct DoVerifyToTarget;

impl<Client> TargetHeightVerifier<Client, VerifyToTarget> for DoVerifyToTarget
where
    Client: HasLightBlockHeight
        + CanVerifyTargetHeight<VerifyForward>
        + CanVerifyTargetHeight<VerifyBackward>
        + CanQueryLightBlock<GetTrustedOrVerified>
        + CanQueryLightBlock<GetHighestTrustedOrVerifiedBefore>
        + CanQueryLightBlock<GetLowestTrustedOrVerified>
        + CanRaiseError<NoInitialTrustedState>,
{
    async fn verify_target_height(
        client: &mut Client,
        _mode: VerifyToTarget,
        target_height: &Client::Height,
    ) -> Result<Client::LightBlock, Client::Error> {
        if let Some(block) = client.query_light_block(GetTrustedOrVerified, target_height) {
            return Ok(block);
        }

        let highest_block = client
            .query_light_block(GetHighestTrustedOrVerifiedBefore, target_height)
            .or_else(|| client.query_light_block(GetHighestTrustedOrVerifiedBefore, target_height))
            .ok_or_else(|| Client::raise_error(NoInitialTrustedState))?;

        let highest_height = Client::light_block_height(&highest_block);

        if target_height >= highest_height {
            client
                .verify_target_height(VerifyForward, target_height)
                .await
        } else {
            client
                .verify_target_height(VerifyBackward, target_height)
                .await
        }
    }
}
