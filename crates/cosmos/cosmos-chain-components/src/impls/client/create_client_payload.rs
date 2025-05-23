use hermes_comet_light_client_components::traits::CanFetchLightBlock;
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use hermes_core::chain_type_components::traits::HasChainId;
use hermes_core::relayer_components::chain::traits::{
    CanQueryChainHeight, CanQueryChainStatus, CreateClientPayloadBuilder,
    CreateClientPayloadBuilderComponent, HasCreateClientPayloadOptionsType,
    HasCreateClientPayloadType,
};
use hermes_error::types::HermesError;
use hermes_prelude::*;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::commitment::CommitmentRoot;
use ibc::core::commitment_types::specs::ProofSpecs;
use ibc::core::host::types::identifiers::ChainId;
use ibc_client_tendermint::types::error::TendermintClientError;
use ibc_client_tendermint::types::{AllowUpdate, ClientState, ConsensusState};
use tendermint::block::Height as TendermintHeight;
use tendermint::error::Error as TendermintError;
use tendermint_rpc::{Client, Error as TendermintRpcError};

use crate::traits::{CanQueryUnbondingPeriod, HasRpcClient};
use crate::types::{
    ChainStatus, CosmosCreateClientOptions, CosmosCreateClientPayload, TendermintClientState,
};

pub struct BuildCosmosCreateClientPayload;

#[cgp_provider(CreateClientPayloadBuilderComponent)]
impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildCosmosCreateClientPayload
where
    Chain: HasRpcClient
        + HasCreateClientPayloadOptionsType<
            Counterparty,
            CreateClientPayloadOptions = CosmosCreateClientOptions,
        > + HasCreateClientPayloadType<Counterparty, CreateClientPayload = CosmosCreateClientPayload>
        + CanQueryUnbondingPeriod
        + HasChainId<ChainId = ChainId>
        + CanQueryChainHeight<Height = Height>
        + CanQueryChainStatus<ChainStatus = ChainStatus>
        + CanRaiseAsyncError<TendermintError>
        + CanRaiseAsyncError<TendermintRpcError>
        + CanRaiseAsyncError<TendermintClientError>
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<HermesError>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &Chain::CreateClientPayloadOptions,
    ) -> Result<CosmosCreateClientPayload, Chain::Error> {
        let latest_height = chain.query_chain_height().await?;

        let unbonding_period = chain.query_unbonding_period().await?;

        let trusting_period = create_client_options.trusting_period;
        let trust_threshold = create_client_options
            .trust_threshold
            .try_into()
            .map_err(|e| {
                Chain::raise_error(format!(
                    "failed to convert `Fraction` to  `TrustThreshold` for trust_threshold: {e}"
                ))
            })?;

        #[allow(deprecated)]
        let client_state = ClientState::new(
            chain.chain_id().clone(),
            trust_threshold,
            trusting_period,
            unbonding_period,
            create_client_options.max_clock_drift,
            latest_height,
            ProofSpecs::cosmos(),
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

        let light_client_options = TendermintClientState::from(client_state.clone())
            .as_light_client_options()
            .map_err(Chain::raise_error)?;

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
