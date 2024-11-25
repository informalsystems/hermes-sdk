use cgp::core::error::CanRaiseError;
use core::time::Duration;
use eyre::Report;
use hermes_chain_type_components::traits::fields::chain_id::HasChainId;
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
use ibc_relayer_types::clients::ics07_tendermint::client_state::{AllowUpdate, ClientState};
use ibc_relayer_types::clients::ics07_tendermint::consensus_state::ConsensusState;
use ibc_relayer_types::clients::ics07_tendermint::error::Error as TendermintClientError;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics23_commitment::commitment::CommitmentRoot;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tendermint::node;
use tendermint_light_client::components::io::{AtHeight, Io, ProdIo};
use tendermint_light_client::light_client::LightClient;
use tendermint_light_client::state::State as LightClientState;
use tendermint_light_client::store::memory::MemoryStore;
use tendermint_light_client::store::LightStore;
use tendermint_light_client::types::Status;
use tendermint_rpc::Client;

use tendermint_light_client::components::clock::FixedClock;
use tendermint_light_client::components::scheduler::basic_bisecting_schedule;
use tendermint_light_client::verifier::ProdVerifier;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::traits::rpc_client::HasRpcClient;
use crate::traits::unbonding_period::CanQueryUnbondingPeriod;
use crate::types::payloads::client::CosmosCreateClientPayload;
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
    Chain: HasCreateClientPayloadOptionsType<Counterparty, CreateClientPayloadOptions = Settings>
        + HasCreateClientPayloadType<Counterparty, CreateClientPayload = CosmosCreateClientPayload>
        + CanQueryUnbondingPeriod<UnbondingPeriod = Duration>
        + HasChainId<ChainId = ChainId>
        + CanQueryChainHeight<Height = Height>
        + CanQueryChainStatus<ChainStatus = ChainStatus>
        + HasRpcClient
        + CanRaiseError<TendermintClientError>
        + CanRaiseError<Report>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &Settings,
    ) -> Result<CosmosCreateClientPayload, Chain::Error> {
        let latest_height = chain.query_chain_height().await?;

        let unbonding_period = chain.query_unbonding_period().await?;

        // Should we use a value for `trusting_period` in the config if it is not
        // found in the client settings?
        // And if both are missing, should we default to another value?
        let trusting_period = create_client_options
            .trusting_period
            .ok_or_else(|| Report::msg("missing trusting period in client settings"))
            .map_err(Chain::raise_error)?;

        // Build the client state.
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

        // Build the consensus state.
        let now = chain.query_chain_status().await?.time;

        let clock = FixedClock::new(now);
        let verifier = ProdVerifier::default();
        let scheduler = basic_bisecting_schedule;

        let rpc_client = chain.rpc_client().clone();

        // Fetch Node info
        let status: node::Info = rpc_client.status().await.map(|s| s.node_info).unwrap();

        let io = ProdIo::new(status.id, rpc_client.clone(), None);

        let trusted_block = io
            .fetch_light_block(AtHeight::At(latest_height.into()))
            .unwrap();

        let client = LightClient::new(
            status.id,
            client_state.as_light_client_options(),
            clock,
            scheduler,
            verifier,
            io,
        );

        let mut store = MemoryStore::new();
        store.insert(trusted_block, Status::Trusted);

        let mut state = LightClientState::new(store);

        // Veriffied light block for consensus state
        let verified_light_block = client
            .verify_to_target(latest_height.into(), &mut state)
            .unwrap();

        let consensus_state = ConsensusState {
            root: CommitmentRoot::from_bytes(
                verified_light_block.signed_header.header.app_hash.as_ref(),
            ),
            timestamp: verified_light_block.signed_header.header.time,
            next_validators_hash: verified_light_block
                .signed_header
                .header
                .next_validators_hash,
        };

        // Create client payload
        Ok(CosmosCreateClientPayload {
            client_state,
            consensus_state,
        })
    }
}
