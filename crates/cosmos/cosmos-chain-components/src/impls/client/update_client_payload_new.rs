use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::core::client::types::Height;
use ibc_client_tendermint::types::error::TendermintClientError;
use ibc_client_tendermint::types::ClientState;
use tendermint::trust_threshold::TrustThresholdFraction;
use tendermint_light_client::components::clock::FixedClock;
use tendermint_light_client::components::io::ProdIo;
use tendermint_light_client::components::scheduler::basic_bisecting_schedule;
use tendermint_light_client::light_client::LightClient;
use tendermint_light_client_verifier::ProdVerifier;
use tendermint_rpc::Client;

use crate::traits::rpc_client::HasRpcClient;

pub struct BuildTendermintUpdateClientPayload;

impl<Chain, Counterparty> UpdateClientPayloadBuilder<Chain, Counterparty>
    for BuildTendermintUpdateClientPayload
where
    Chain: HasHeightType<Height = Height>
        + CanQueryChainStatus
        + HasRpcClient
        + HasUpdateClientPayloadType<Counterparty>
        + HasClientStateType<Counterparty, ClientState = ClientState>
        + CanRaiseError<tendermint::Error>
        + CanRaiseError<tendermint_rpc::Error>
        + CanRaiseError<TendermintClientError>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &Chain::Height,
        target_height: &Chain::Height,
        client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        let rpc_client = chain.rpc_client();
        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let clock = FixedClock::new(current_time);

        let verifier = ProdVerifier::default();

        let trust_threshold = TrustThresholdFraction::new(
            client_state.trust_level.numerator(),
            client_state.trust_level.denominator(),
        )
        .map_err(Chain::raise_error)?;

        let light_client_options = client_state
            .as_light_client_options()
            .map_err(Chain::raise_error)?;

        let io = ProdIo::new(peer_id, rpc_client.clone(), None);

        // let trusted_block_height = trusted_height.revision_height().try_into()
        //     .map_err(Chain::raise_error)?;

        // let block = io.fetch_light_block(AtHeight::At(trusted_block_height));

        let light_client = LightClient::new(
            peer_id,
            light_client_options,
            clock,
            basic_bisecting_schedule,
            verifier,
            io,
        );

        todo!()
    }
}
