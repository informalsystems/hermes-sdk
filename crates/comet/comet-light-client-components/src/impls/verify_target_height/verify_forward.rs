use core::fmt::Debug;

use hermes_chain_type_components::traits::HasHeightType;
use hermes_prelude::*;

use crate::traits::{
    CanComputeNextVerificationHeight, CanFetchLightBlockWithStatus, CanQueryLightBlock,
    CanTraceVerificationHeight, CanUpdateVerificationStatus, CanValidateLightBlock,
    CanVerifyUpdateHeader, GetHighestTrustedOrVerifiedBefore, HasLightBlockHeight, HasVerdictType,
    HasVerificationStatusType, IsWithinTrustingPeriod, NoInitialTrustedState, TargetHeightVerifier,
    TargetHeightVerifierComponent, VerifiedStatus,
};
use crate::types::{Verdict, VerificationStatus};

pub struct DoVerifyForward;

pub struct TargetLowerThanTrustedHeight<'a, Client>
where
    Client: HasHeightType,
{
    pub target_height: &'a Client::Height,
    pub trusted_height: &'a Client::Height,
}

#[cgp_provider(TargetHeightVerifierComponent)]
impl<Client, Mode> TargetHeightVerifier<Client, Mode> for DoVerifyForward
where
    Client: HasLightBlockHeight
        + HasVerdictType<Verdict = Verdict>
        + HasVerificationStatusType<VerificationStatus = VerificationStatus>
        + CanVerifyUpdateHeader
        + CanTraceVerificationHeight
        + CanFetchLightBlockWithStatus
        + CanComputeNextVerificationHeight
        + CanUpdateVerificationStatus<VerifiedStatus>
        + CanValidateLightBlock<IsWithinTrustingPeriod>
        + CanQueryLightBlock<GetHighestTrustedOrVerifiedBefore>
        + CanRaiseAsyncError<NoInitialTrustedState>
        + for<'a> CanRaiseAsyncError<TargetLowerThanTrustedHeight<'a, Client>>,
    Mode: Async,
{
    async fn verify_target_height(
        client: &mut Client,
        _mode: Mode,
        target_height: &Client::Height,
    ) -> Result<Client::LightBlock, Client::Error> {
        let mut current_height = target_height.clone();

        loop {
            let trusted_block = client
                .query_light_block(GetHighestTrustedOrVerifiedBefore, target_height)
                .ok_or_else(|| Client::raise_error(NoInitialTrustedState))?;

            let trusted_height = Client::light_block_height(&trusted_block);

            if target_height < trusted_height {
                return Err(Client::raise_error(TargetLowerThanTrustedHeight {
                    target_height,
                    trusted_height,
                }));
            }

            client.validate_light_block(IsWithinTrustingPeriod, &trusted_block)?;

            client.trace_verification_height(target_height, &current_height);

            if target_height == trusted_height {
                return Ok(trusted_block);
            }

            let (current_block, current_status) = client
                .fetch_light_block_with_status(&current_height)
                .await?;

            let verdict = client.verify_update_header(&current_block, &trusted_block)?;

            if verdict == Verdict::Success {
                if current_status == VerificationStatus::Unverified {
                    client.update_verification_status(VerifiedStatus, &current_block);
                }

                client.trace_verification_height(&current_height, trusted_height);
            }

            current_height =
                client.compute_next_verification_height(&current_height, target_height)?;
        }
    }
}

impl<Client> Debug for TargetLowerThanTrustedHeight<'_, Client>
where
    Client: HasHeightType<Height: Debug>,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TargetLowerThanTrustedHeight")
            .field("target_height", &self.target_height)
            .field("trusted_height", &self.trusted_height)
            .finish()
    }
}
