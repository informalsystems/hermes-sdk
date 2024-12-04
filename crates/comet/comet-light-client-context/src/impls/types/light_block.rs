use cgp::core::Async;
use hermes_chain_components::traits::types::height::HasHeightType;
use hermes_comet_light_client_components::traits::light_block::height::LightBlockHeightGetter;
use hermes_comet_light_client_components::traits::types::light_block::{
    HasLightBlockType, ProvideLightBlockType,
};
use tendermint::block::Height;
use tendermint_light_client_verifier::types::LightBlock;

pub struct UseTendermintLightBlock;

impl<Client: Async> ProvideLightBlockType<Client> for UseTendermintLightBlock {
    type LightBlock = LightBlock;
}

impl<Client> LightBlockHeightGetter<Client> for UseTendermintLightBlock
where
    Client: HasLightBlockType<LightBlock = LightBlock> + HasHeightType<Height = Height>,
{
    fn light_block_height(light_block: &LightBlock) -> &Height {
        &light_block.signed_header.header.height
    }
}
