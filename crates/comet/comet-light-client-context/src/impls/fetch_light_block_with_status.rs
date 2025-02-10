use cgp::prelude::*;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_comet_light_client_components::traits::fetch_light_block::{
    CanFetchLightBlock, LightBlockWithStatusFetcher, LightBlockWithStatusFetcherComponent,
};
use hermes_comet_light_client_components::traits::types::light_block::HasLightBlockType;
use hermes_comet_light_client_components::traits::types::status::HasVerificationStatusType;
use hermes_comet_light_client_components::types::status::VerificationStatus;
use tendermint::block::Height;
use tendermint_light_client_verifier::types::LightBlock;

use crate::traits::light_block_store::HasLightBlockStore;

pub struct FetchTendermintLightBlockWithStatus;

#[cgp_provider(LightBlockWithStatusFetcherComponent)]
impl<Client> LightBlockWithStatusFetcher<Client> for FetchTendermintLightBlockWithStatus
where
    Client: HasHeightType<Height = Height>
        + HasLightBlockType<LightBlock = LightBlock>
        + HasVerificationStatusType<VerificationStatus = VerificationStatus>
        + CanFetchLightBlock
        + HasLightBlockStore,
{
    async fn fetch_light_block_with_status(
        client: &mut Client,
        height: &Height,
    ) -> Result<(LightBlock, VerificationStatus), Client::Error> {
        let m_entry = client.light_block_store().get(height);

        if let Some((block, status)) = m_entry {
            return Ok((block.clone(), *status));
        }

        let block = client.fetch_light_block(height).await?;

        let entry = (block, VerificationStatus::Unverified);

        client
            .light_block_store_mut()
            .insert(*height, entry.clone());

        Ok(entry)
    }
}
