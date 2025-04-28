use hermes_chain_components::traits::HasHeightType;
use hermes_comet_light_client_components::traits::{
    GetHighestTrustedOrVerifiedBefore, HasLightBlockType, LightBlockQuerier,
    LightBlockQuerierComponent,
};
use hermes_comet_light_client_components::types::VerificationStatus;
use hermes_prelude::*;
use tendermint::block::Height;
use tendermint_light_client_verifier::types::LightBlock;

use crate::traits::light_block_store::HasLightBlockStore;

pub struct QueryHighestTrustedOrVerifiedBefore;

#[cgp_provider(LightBlockQuerierComponent)]
impl<Client> LightBlockQuerier<Client, GetHighestTrustedOrVerifiedBefore>
    for QueryHighestTrustedOrVerifiedBefore
where
    Client: HasHeightType<Height = Height>
        + HasLightBlockType<LightBlock = LightBlock>
        + HasLightBlockStore,
{
    fn query_light_block(
        client: &Client,
        _mode: GetHighestTrustedOrVerifiedBefore,
        target_height: &Height,
    ) -> Option<LightBlock> {
        client
            .light_block_store()
            .iter()
            .filter(|(height, (_, status))| {
                height <= &target_height
                    && (status == &VerificationStatus::Verified
                        || status == &VerificationStatus::Trusted)
            })
            .max_by_key(|(height, _)| *height)
            .map(|(_, (block, _))| block.clone())
    }
}
