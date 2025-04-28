use hermes_chain_components::traits::HasHeightType;
use hermes_comet_light_client_components::traits::{
    HasLightBlockType, LightBlockFetcher, LightBlockFetcherComponent,
};
use hermes_prelude::*;
use tendermint_light_client_verifier::types::{Height, LightBlock, ValidatorSet};
use tendermint_rpc::{Client, Error as RpcError, Paging};

use crate::traits::peer_id::HasPeerId;
use crate::traits::rpc_client::HasRpcClient;

pub struct FetchTendermintLightBlock;

#[cgp_provider(LightBlockFetcherComponent)]
impl<Client> LightBlockFetcher<Client> for FetchTendermintLightBlock
where
    Client: HasHeightType<Height = Height>
        + HasLightBlockType<LightBlock = LightBlock>
        + HasRpcClient
        + HasPeerId
        + CanRaiseAsyncError<tendermint::Error>
        + CanRaiseAsyncError<RpcError>,
{
    async fn fetch_light_block(
        client: &Client,
        height: &Height,
    ) -> Result<Client::LightBlock, Client::Error> {
        let peer_id = client.peer_id();
        let rpc_client = client.rpc_client();

        let signed_header = rpc_client
            .commit(*height)
            .await
            .map_err(Client::raise_error)?
            .signed_header;

        let height = signed_header.header.height;
        let proposer_address = signed_header.header.proposer_address;

        let validators = rpc_client
            .validators(height, Paging::All)
            .await
            .map_err(Client::raise_error)?
            .validators;

        let validator_set = ValidatorSet::with_proposer(validators, proposer_address)
            .map_err(Client::raise_error)?;

        let next_validators = rpc_client
            .validators(height.increment(), Paging::All)
            .await
            .map_err(Client::raise_error)?
            .validators;

        let next_validator_set = ValidatorSet::without_proposer(next_validators);

        let light_block =
            LightBlock::new(signed_header, validator_set, next_validator_set, *peer_id);

        Ok(light_block)
    }
}
