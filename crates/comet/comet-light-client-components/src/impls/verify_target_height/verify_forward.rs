use cgp::core::Async;
use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::compute_verification_height::CanComputeNextVerificationHeight;
use crate::traits::fetch_light_block::CanFetchLightBlockWithStatus;
use crate::traits::light_block::height::HasLightBlockHeight;
use crate::traits::light_block::time::HasLightBlockTime;
use crate::traits::state::query_light_block::{
    CanQueryLightBlock, GetHighestTrustedOrVerifiedBefore, GetLowestTrustedOrVerified,
    GetTrustedOrVerified,
};
use crate::traits::trace_light_block::CanTraceLightBlock;
use crate::traits::types::status::HasVerificationStatusType;
use crate::traits::types::verdict::HasVerdictType;
use crate::traits::update_verification_status::{CanUpdateVerificationStatus, VerifiedStatus};
use crate::traits::validate_light_block::{CanValidateLightBlock, IsWithinTrustingPeriod};
use crate::traits::verify_target_height::{NoInitialTrustedState, TargetHeightVerifier};
use crate::traits::verify_update_header::CanVerifyUpdateHeader;
use crate::types::status::VerificationStatus;
use crate::types::verdict::Verdict;

pub struct DoVerifyForward;

pub struct TargetLowerThanTrustedHeight<'a, Chain>
where
    Chain: HasHeightType,
{
    pub target_height: &'a Chain::Height,
    pub trusted_height: &'a Chain::Height,
}

impl<Chain, Mode> TargetHeightVerifier<Chain, Mode> for DoVerifyForward
where
    Chain: HasLightBlockHeight
        + HasLightBlockTime
        + HasVerdictType<Verdict = Verdict>
        + HasVerificationStatusType<VerificationStatus = VerificationStatus>
        + CanVerifyUpdateHeader
        + CanTraceLightBlock
        + CanFetchLightBlockWithStatus
        + CanComputeNextVerificationHeight
        + CanUpdateVerificationStatus<VerifiedStatus>
        + CanValidateLightBlock<IsWithinTrustingPeriod>
        + CanQueryLightBlock<GetTrustedOrVerified>
        + CanQueryLightBlock<GetHighestTrustedOrVerifiedBefore>
        + CanQueryLightBlock<GetLowestTrustedOrVerified>
        + CanRaiseError<NoInitialTrustedState>
        + for<'a> CanRaiseError<TargetLowerThanTrustedHeight<'a, Chain>>,
    Mode: Async,
{
    async fn verify_target_height(
        chain: &Chain,
        _mode: Mode,
        state: &mut Chain::VerifierState,
        target_height: &Chain::Height,
    ) -> Result<Chain::LightBlock, Chain::Error> {
        let mut current_height = target_height.clone();

        loop {
            let trusted_block =
                Chain::query_light_block(GetHighestTrustedOrVerifiedBefore, state, target_height)
                    .ok_or_else(|| Chain::raise_error(NoInitialTrustedState))?;

            let trusted_height = Chain::light_block_height(&trusted_block);

            if target_height < trusted_height {
                return Err(Chain::raise_error(TargetLowerThanTrustedHeight {
                    target_height,
                    trusted_height,
                }));
            }

            chain.validate_light_block(IsWithinTrustingPeriod, &trusted_block)?;

            Chain::trace_light_block(state, target_height, &current_height);

            if target_height == trusted_height {
                return Ok(trusted_block);
            }

            let (current_block, current_status) = chain
                .fetch_light_block_with_status(&current_height, state)
                .await?;

            let verdict = chain.verify_update_header(&current_block, &trusted_block)?;

            if verdict == Verdict::Success {
                if current_status == VerificationStatus::Unverified {
                    Chain::update_verification_status(state, &current_height, VerifiedStatus);
                }

                Chain::trace_light_block(state, &current_height, trusted_height);
            }

            current_height =
                Chain::compute_next_verification_height(state, &current_height, target_height);
        }
    }
}
