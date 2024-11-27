use cgp::core::error::CanRaiseError;
use core::time::Duration;
use eyre::Report;
use hermes_comet_light_client_components::traits::fetch_light_block::CanFetchLightBlock;
use hermes_comet_light_client_context::contexts::light_client::CometLightClient;
use ibc::core::client::types::Height;
use tendermint::trust_threshold::TrustThresholdFraction;
use tendermint_light_client::light_client::Options;

use hermes_chain_type_components::traits::fields::chain_id::HasChainId;
use hermes_error::types::HermesError;
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::{
    CanQueryChainHeight, CanQueryChainStatus,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};

use ibc_relayer::chain::client::ClientSettings;
use ibc_relayer::chain::cosmos::client::Settings;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer::consensus_state::AnyConsensusState;

use tendermint::block::Height as TendermintHeight;
use tendermint::error::Error as TendermintError;
use tendermint_proto::google::protobuf::Duration as ProtoDuration;
use tendermint_rpc::Client;
use tendermint_rpc::Error as TendermintRpcError;

use ibc::core::host::types::identifiers::ChainId;

use ibc_proto::ibc::core::commitment::v1::MerkleRoot;
use ibc_proto::ibc::lightclients::tendermint::v1::ClientState as ProtoClientState;
use ibc_proto::ibc::lightclients::tendermint::v1::ConsensusState as ProtoConsensusState;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::traits::rpc_client::HasRpcClient;
use crate::traits::unbonding_period::CanQueryUnbondingPeriod;
use crate::types::payloads::client::CosmosCreateClientOptions;
use crate::types::payloads::client::{CosmosCreateClientPayload, CosmosCreateClientPayloadV2};
use crate::types::status::ChainStatus;

pub struct BuildCreateClientPayloadWithChainHandle;

impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildCreateClientPayloadWithChainHandle
where
    Chain: HasCreateClientPayloadOptionsType<Counterparty, CreateClientPayloadOptions = Settings>
        + HasCreateClientPayloadType<Counterparty, CreateClientPayload = CosmosCreateClientPayload>
        + HasBlockingChainHandle
        + CanRaiseError<eyre::Report>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &Settings,
    ) -> Result<CosmosCreateClientPayload, Chain::Error> {
        let client_settings = create_client_options.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let height = chain_handle
                    .query_latest_height()
                    .map_err(Chain::raise_error)?;

                let any_client_state = chain_handle
                    .build_client_state(height, ClientSettings::Tendermint(client_settings))
                    .map_err(Chain::raise_error)?;

                let client_state = match &any_client_state {
                    AnyClientState::Tendermint(client_state) => client_state.clone(),
                };

                let any_consensus_state = chain_handle
                    .build_consensus_state(
                        any_client_state.latest_height(),
                        height,
                        any_client_state,
                    )
                    .map_err(Chain::raise_error)?;

                let AnyConsensusState::Tendermint(consensus_state) = any_consensus_state;

                Ok(CosmosCreateClientPayload {
                    client_state,
                    consensus_state,
                })
            })
            .await
    }
}

pub struct BuildCreateClientPayload;

impl<Chain, Counterparty> CreateClientPayloadBuilder<Chain, Counterparty>
    for BuildCreateClientPayload
where
    Chain: HasCreateClientPayloadOptionsType<
            Counterparty,
            CreateClientPayloadOptions = CosmosCreateClientOptions,
        > + HasCreateClientPayloadType<Counterparty, CreateClientPayload = CosmosCreateClientPayloadV2>
        + CanQueryUnbondingPeriod<UnbondingPeriod = Duration>
        + HasChainId<ChainId = ChainId>
        + CanQueryChainHeight<Height = Height>
        + CanQueryChainStatus<ChainStatus = ChainStatus>
        + HasRpcClient
        + CanRaiseError<TendermintError>
        + CanRaiseError<TendermintRpcError>
        + CanRaiseError<Report>
        + CanRaiseError<HermesError>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &CosmosCreateClientOptions,
    ) -> Result<CosmosCreateClientPayloadV2, Chain::Error> {
        let latest_height = chain.query_chain_height().await?;

        let unbonding_period = chain.query_unbonding_period().await?;

        // Should we use a value for `trusting_period` in the config if it is not
        // found in the client settings?
        // And if both are missing, should we default to another value?
        let trusting_period = create_client_options
            .trusting_period
            .map(|trusting_period| {
                trusting_period.try_into().map_err(|e| {
                    Chain::raise_error(Report::msg(format!(
                        "Failed to convert create_client_options to tendermint-proto Duration: {e}"
                    )))
                })
            })
            .transpose()?;

        let max_clock_drift: Option<ProtoDuration> =
            create_client_options.max_clock_drift.try_into().ok();
        let unbonding_period = unbonding_period.try_into().map_err(|e| {
            Chain::raise_error(Report::msg(format!(
                "Failed to convert create_client_options to tendermint-proto Duration: {e}"
            )))
        })?;
        let trust_threshold = create_client_options.trust_threshold;

        #[allow(deprecated)]
        let client_state = ProtoClientState {
            chain_id: chain.chain_id().to_string(),
            trust_level: Some(trust_threshold),
            trusting_period,
            unbonding_period: Some(unbonding_period),
            max_clock_drift,
            frozen_height: None,
            latest_height: Some(latest_height.into()),
            proof_specs: Default::default(),
            upgrade_path: vec!["upgrade".to_string(), "upgradedIBCState".to_string()],
            allow_update_after_expiry: true,
            allow_update_after_misbehaviour: true,
        };

        let rpc_client = chain.rpc_client().clone();

        let tendermint_latest_height = TendermintHeight::try_from(latest_height.revision_height())
            .map_err(Chain::raise_error)?;

        let status = rpc_client.status().await.map_err(Chain::raise_error)?;

        let current_time = status.sync_info.latest_block_time;
        let peer_id = status.node_info.id;

        let light_client_options = Options {
            trust_threshold: TrustThresholdFraction::new(
                trust_threshold.numerator,
                trust_threshold.denominator,
            )
            .map_err(Chain::raise_error)?,
            trusting_period: create_client_options.trusting_period.unwrap(),
            clock_drift: create_client_options.max_clock_drift,
        };

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

        let timestamp = trusted_block.signed_header.header.time;

        let consensus_state = ProtoConsensusState {
            timestamp: Some(timestamp.into()),
            root: Some(MerkleRoot {
                hash: trusted_block
                    .signed_header
                    .header
                    .app_hash
                    .as_ref()
                    .to_vec(),
            }),
            next_validators_hash: trusted_block
                .signed_header
                .header
                .next_validators_hash
                .as_bytes()
                .to_vec(),
        };

        // Create client payload
        Ok(CosmosCreateClientPayloadV2 {
            client_state,
            consensus_state,
        })
    }
}
