use cgp::prelude::*;
use hermes_chain_type_components::traits::HasHeightType;
use hermes_comet_light_client_components::traits::update_client::CanBuildLightBlocksForUpdateClient;
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_error::types::HermesError;
use hermes_relayer_components::chain::traits::{
    CanQueryChainStatus, HasClientStateType, HasUpdateClientPayloadType,
    UpdateClientPayloadBuilder, UpdateClientPayloadBuilderComponent,
};
use ibc::clients::tendermint::types::error::TendermintClientError;
use ibc::clients::tendermint::types::Header;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use tendermint::block::Height as TmHeight;
use tendermint_rpc::Client;

use crate::traits::rpc_client::HasRpcClient;
use crate::types::payloads::client::CosmosUpdateClientPayload;
use crate::types::tendermint::TendermintClientState;

pub struct BuildTendermintUpdateClientPayload;

#[cgp_provider(UpdateClientPayloadBuilderComponent)]
impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildTendermintUpdateClientPayload
where
    Chain: HasHeightType<Height = Height>
        + CanQueryChainStatus
        + HasRpcClient
        + HasUpdateClientPayloadType<Counterparty, UpdateClientPayload = CosmosUpdateClientPayload>
        + HasClientStateType<Counterparty>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<tendermint::Error>
        + CanRaiseAsyncError<tendermint_rpc::Error>
        + CanRaiseAsyncError<ClientError>
        + CanRaiseAsyncError<TendermintClientError>
        + CanRaiseAsyncError<HermesError>,
    TendermintClientState: From<Chain::ClientState>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &Height,
        target_height: &Height,
        client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let rpc_client = chain.rpc_client();
        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let light_client_options = TendermintClientState::from(client_state)
            .as_light_client_options()
            .map_err(Chain::raise_error)?;

        let mut light_client = CometLightClient::new(
            current_time,
            peer_id,
            rpc_client.clone(),
            light_client_options,
        );

        let trusted_tm_height =
            TmHeight::try_from(trusted_height.revision_height()).map_err(Chain::raise_error)?;

        let target_tm_height =
            TmHeight::try_from(target_height.revision_height()).map_err(Chain::raise_error)?;

        let blocks = light_client
            .build_light_blocks_for_update_client(&trusted_tm_height, &target_tm_height)
            .await
            .map_err(Chain::raise_error)?;

        let mut target_blocks = blocks.iter();
        target_blocks.next();

        let revision_number = target_height.revision_number();

        let headers = target_blocks
            .zip(blocks.iter())
            .map(|(target_block, trusted_block)| {
                let trusted_height = Height::new(revision_number, trusted_block.height().into())
                    .map_err(Chain::raise_error)?;

                Ok(Header {
                    signed_header: target_block.signed_header.clone(),
                    validator_set: target_block.validators.clone(),
                    trusted_height,
                    trusted_next_validator_set: trusted_block.validators.clone(),
                })
            })
            .collect::<Result<Vec<_>, Chain::Error>>()?;

        Ok(CosmosUpdateClientPayload { headers })
    }
}
