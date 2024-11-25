use std::collections::BTreeMap;

use cgp::prelude::*;
use hermes_comet_light_client_components::types::status::VerificationStatus;
use tendermint::block::Height;
use tendermint_light_client_verifier::types::LightBlock;

pub trait HasLightBlockStore: Async {
    fn light_block_store(&self) -> &BTreeMap<Height, (LightBlock, VerificationStatus)>;

    fn light_block_store_mut(&mut self) -> &mut BTreeMap<Height, (LightBlock, VerificationStatus)>;
}
