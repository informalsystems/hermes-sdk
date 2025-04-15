use std::collections::btree_map::Entry;

use cgp::prelude::*;
use hermes_comet_light_client_components::traits::{
    HasLightBlockType, TrustedStatus, VerificationStatusUpdater,
    VerificationStatusUpdaterComponent, VerifiedStatus,
};
use hermes_comet_light_client_components::types::VerificationStatus;
use tendermint_light_client_verifier::types::LightBlock;

use crate::traits::light_block_store::HasLightBlockStore;

pub struct DoUpdateVerifactionStatus;

#[cgp_provider(VerificationStatusUpdaterComponent)]
impl<Client> VerificationStatusUpdater<Client, VerifiedStatus> for DoUpdateVerifactionStatus
where
    Client: HasLightBlockType<LightBlock = LightBlock> + HasLightBlockStore,
{
    fn update_verification_status(
        client: &mut Client,
        _status: VerifiedStatus,
        block: &LightBlock,
    ) {
        let height = block.height();

        let entry = client.light_block_store_mut().entry(height);

        match entry {
            Entry::Occupied(mut entry) => {
                let (_, status) = entry.get_mut();

                if status == &VerificationStatus::Unverified {
                    *status = VerificationStatus::Verified;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert((block.clone(), VerificationStatus::Verified));
            }
        }
    }
}

#[cgp_provider(VerificationStatusUpdaterComponent)]
impl<Client> VerificationStatusUpdater<Client, TrustedStatus> for DoUpdateVerifactionStatus
where
    Client: HasLightBlockType<LightBlock = LightBlock> + HasLightBlockStore,
{
    fn update_verification_status(client: &mut Client, _status: TrustedStatus, block: &LightBlock) {
        let height = block.height();

        client
            .light_block_store_mut()
            .insert(height, (block.clone(), VerificationStatus::Trusted));
    }
}
