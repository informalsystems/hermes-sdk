use cgp::core::error::CanRaiseError;

use hermes_chain_type_components::traits::fields::chain_id::HasChainId;
use hermes_comet_light_client_components::traits::fetch_light_block::CanFetchLightBlock;
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_error::types::HermesError;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};

use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer_types::clients::ics07_tendermint::client_state::{AllowUpdate, ClientState};
use ibc_relayer_types::clients::ics07_tendermint::consensus_state::ConsensusState;
use ibc_relayer_types::clients::ics07_tendermint::error::Error as TendermintClientError;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::Height;

use tendermint::block::Height as TendermintHeight;
use tendermint::error::Error as TendermintError;
use tendermint_rpc::Client;
use tendermint_rpc::Error as TendermintRpcError;

use crate::traits::rpc_client::HasRpcClient;
use crate::traits::unbonding_period::CanQueryUnbondingPeriod;
use crate::types::payloads::client::CosmosCreateClientPayload;
use crate::types::status::ChainStatus;
use crate::types::tendermint::TendermintClientState;

pub struct BuildCosmosCreateClientPayload;

impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildCosmosCreateClientPayload
where
    Chain: HasRpcClient
        + HasCreateClientPayloadOptionsType<Counterparty, CreateClientPayloadOptions = Settings>
        + HasCreateClientPayloadType<Counterparty, CreateClientPayload = CosmosCreateClientPayload>
        + CanQueryUnbondingPeriod
        + HasChainId<ChainId = ChainId>
        + CanQueryChainHeight<Height = Height>
        + CanQueryChainStatus<ChainStatus = ChainStatus>
        + CanRaiseError<TendermintClientError>
        + CanRaiseError<TendermintError>
        + CanRaiseError<TendermintRpcError>
        + CanRaiseError<&'static str>
        + CanRaiseError<HermesError>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &Settings,
    ) -> Result<CosmosCreateClientPayload, Chain::Error> {
        let latest_height = chain.query_chain_height().await?;

        let unbonding_period = chain.query_unbonding_period().await?;

        // TODO: Should we use a value for `trusting_period` in the config if it is not
        // found in the client settings?
        // And if both are missing, should we default to another value?
        // Current behaviour is to default to 2/3 of unbonding period if the
        // trusting period is missing from the client settings.
        let trusting_period = create_client_options
            .trusting_period
            .unwrap_or(2 * unbonding_period / 3);

        #[allow(deprecated)]
        let client_state = ClientState::new(
            chain.chain_id().clone(),
            create_client_options.trust_threshold,
            trusting_period,
            unbonding_period,
            create_client_options.max_clock_drift,
            latest_height,
            Default::default(),
            vec!["upgrade".to_string(), "upgradedIBCState".to_string()],
            AllowUpdate {
                after_expiry: true,
                after_misbehaviour: true,
            },
        )
        .map_err(Chain::raise_error)?;

        let rpc_client = chain.rpc_client().clone();

        let tendermint_latest_height = TendermintHeight::try_from(latest_height.revision_height())
            .map_err(Chain::raise_error)?;

        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let light_client_options =
            TendermintClientState::from(client_state.clone()).as_light_client_options();

        let light_client = CometLightClient::new(
            current_time,
            peer_id,
            rpc_client.clone(),
            light_client_options,
        );

        let trusted_block = light_client
            .fetch_light_block(&tendermint_latest_height)
            .await
            .map_err(Chain::raise_error)?;

        let consensus_state = ConsensusState {
            root: CommitmentRoot::from_bytes(trusted_block.signed_header.header.app_hash.as_ref()),
            timestamp: trusted_block.signed_header.header.time,
            next_validators_hash: trusted_block.signed_header.header.next_validators_hash,
        };

        // Create client payload
        Ok(CosmosCreateClientPayload {
            client_state,
            consensus_state,
        })
    }
}
