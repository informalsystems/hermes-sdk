use cgp::prelude::*;
use hermes_chain_components::traits::HasHeightType;
use hermes_comet_light_client_components::traits::{
    CanFetchLightBlock, CanUpdateVerificationStatus, CanVerifyTargetHeight, HasLightBlockType,
    LightBlocksForUpdateClientBuilder, LightBlocksForUpdateClientBuilderComponent, TrustedStatus,
    VerifyForward,
};
use hermes_comet_light_client_components::types::VerificationStatus;
use tendermint::block::Height;
use tendermint_light_client_verifier::types::LightBlock;

use crate::traits::light_block_store::HasLightBlockStore;
use crate::traits::verification_trace::HasVerificationTrace;

pub struct BuildTendermintUpdateClientBlocks;

#[cgp_provider(LightBlocksForUpdateClientBuilderComponent)]
impl<Client> LightBlocksForUpdateClientBuilder<Client> for BuildTendermintUpdateClientBlocks
where
    Client: HasHeightType<Height = Height>
        + HasLightBlockType<LightBlock = LightBlock>
        + CanFetchLightBlock
        + CanUpdateVerificationStatus<TrustedStatus>
        + CanVerifyTargetHeight<VerifyForward>
        + HasVerificationTrace
        + HasLightBlockStore,
{
    async fn build_light_blocks_for_update_client(
        client: &mut Client,
        trusted_height: &Height,
        target_height: &Height,
    ) -> Result<Vec<LightBlock>, Client::Error> {
        let trusted_block = client.fetch_light_block(trusted_height).await?;

        client.update_verification_status(TrustedStatus, &trusted_block);

        let target_block = client
            .verify_target_height(VerifyForward, target_height)
            .await?;

        let m_heights = client.verification_trace().get(target_height);

        if let Some(heights) = m_heights {
            let store = client.light_block_store();

            let blocks = heights
                .iter()
                .filter_map(|height| match store.get(height) {
                    Some((block, VerificationStatus::Trusted)) => Some(block.clone()),
                    Some((block, VerificationStatus::Verified)) => Some(block.clone()),
                    _ => None,
                })
                .collect();

            Ok(blocks)
        } else {
            Ok(vec![trusted_block, target_block])
        }
    }
}
