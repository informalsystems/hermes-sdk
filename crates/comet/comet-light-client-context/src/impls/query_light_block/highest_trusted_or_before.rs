use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_comet_light_client_components::traits::query_light_block::{
    GetHighestTrustedOrVerifiedBefore, LightBlockQuerier,
};
use hermes_comet_light_client_components::traits::types::light_block::HasLightBlockType;
use hermes_comet_light_client_components::types::status::VerificationStatus;
use tendermint::block::Height;
use tendermint_light_client_verifier::types::LightBlock;

use crate::traits::light_block_store::HasLightBlockStore;

pub struct QueryHighestTrustedOrVerifiedBefore;

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
