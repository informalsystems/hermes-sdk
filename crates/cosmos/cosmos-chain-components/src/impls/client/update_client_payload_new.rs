use cgp::prelude::CanRaiseError;
use hermes_comet_light_client_components::traits::verify_target_height::{
    CanVerifyTargetHeight, VerifyForward,
};
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_error::types::HermesError;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_client_tendermint::types::error::TendermintClientError;
use ibc_client_tendermint::types::ClientState;
use tendermint::block::Height as TmHeight;
use tendermint_rpc::Client;

use crate::traits::rpc_client::HasRpcClient;

pub struct BuildTendermintUpdateClientPayload;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildTendermintUpdateClientPayload
where
    Chain: HasHeightFields
        + CanQueryChainStatus
        + HasRpcClient
        + HasUpdateClientPayloadType<Counterparty>
        + HasClientStateType<Counterparty, ClientState = ClientState>
        + CanRaiseError<tendermint::Error>
        + CanRaiseError<tendermint_rpc::Error>
        + CanRaiseError<TendermintClientError>
        + CanRaiseError<HermesError>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        _trusted_height: &Chain::Height,
        target_height: &Chain::Height,
        client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let rpc_client = chain.rpc_client();
        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let light_client_options = client_state
            .as_light_client_options()
            .map_err(Chain::raise_error)?;

        let mut light_client = CometLightClient::new(
            current_time,
            peer_id,
            rpc_client.clone(),
            light_client_options,
        );

        let target_tm_height = TmHeight::try_from(Chain::revision_height(target_height))
            .map_err(Chain::raise_error)?;

        light_client
            .verify_target_height(VerifyForward, &target_tm_height)
            .await
            .map_err(Chain::raise_error)?;

        todo!()
    }
}
