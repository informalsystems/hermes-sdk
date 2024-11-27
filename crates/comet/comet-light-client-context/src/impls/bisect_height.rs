use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_comet_light_client_components::traits::compute_verification_height::NextVerificationHeightComputer;
use hermes_comet_light_client_components::traits::light_block::height::HasLightBlockHeight;
use hermes_comet_light_client_components::traits::query_light_block::{
    CanQueryLightBlock, GetHighestTrustedOrVerifiedBefore,
};
use hermes_comet_light_client_components::traits::verify_target_height::NoInitialTrustedState;
use tendermint::block::Height;

pub struct BisectHeight;

impl<Client> NextVerificationHeightComputer<Client> for BisectHeight
where
    Client: HasHeightType<Height = Height>
        + HasLightBlockHeight
        + CanQueryLightBlock<GetHighestTrustedOrVerifiedBefore>
        + CanRaiseError<NoInitialTrustedState>
        + CanRaiseError<tendermint::Error>,
{
    fn compute_next_verification_height(
        client: &Client,
        current_height: &Client::Height,
        target_height: &Client::Height,
    ) -> Result<Client::Height, Client::Error> {
        let trusted_block = client
            .query_light_block(GetHighestTrustedOrVerifiedBefore, target_height)
            .ok_or_else(|| Client::raise_error(NoInitialTrustedState))?;

        let trusted_height = Client::light_block_height(&trusted_block);

        if trusted_height == target_height {
            Ok(*target_height)
        } else {
            let low = trusted_height.value();
            let high = current_height.value();

            let midpoint = low + (high + 1 - low) / 2;

            let next_height = midpoint.try_into().map_err(Client::raise_error)?;

            Ok(next_height)
        }
    }
}