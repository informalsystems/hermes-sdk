use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use cgp_core::prelude::*;
use eyre::{eyre, Error};
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use hermes_relayer_components::chain::traits::commitment_prefix::IbcCommitmentPrefixGetter;
use hermes_relayer_components::chain::traits::queries::channel_end::ChannelEndQuerier;
use hermes_relayer_components::chain::traits::queries::client_state::ClientStateQuerier;
use hermes_relayer_components::chain::traits::queries::connection_end::ConnectionEndQuerier;
use hermes_relayer_components::chain::traits::queries::consensus_state::ConsensusStateQuerier;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_runtime::types::runtime::HermesRuntime;
use ibc::core::connection::types::ConnectionEnd;
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::Height;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey};

use crate::impls::chain::component::SolomachineChainComponents;
use crate::methods::encode::public_key::PublicKey;
use crate::traits::solomachine::Solomachine;

const DEFAULT_DIVERSIFIER: &str = "solo-machine-diversifier";

#[derive(HasField, Clone)]
pub struct MockSolomachine {
    pub runtime: HermesRuntime,
    pub chain_id: ChainId,
    commitment_prefix: String,
    public_key: PublicKey,
    secret_key: SecretKey,
    client_states: Arc<Mutex<HashMap<ClientId, TendermintClientState>>>,
    client_consensus_states: Arc<Mutex<HashMap<ClientId, TendermintConsensusState>>>,
    pub telemetry: CosmosTelemetry,
    pub connections: Arc<Mutex<HashMap<ConnectionId, ConnectionEnd>>>,
}

impl MockSolomachine {
    pub fn new(
        chain_id: &str,
        commitment_prefix: String,
        runtime: HermesRuntime,
        telemetry: CosmosTelemetry,
    ) -> Self {
        let secp = Secp256k1::new();
        let (secret_key, secp_public_key) = secp.generate_keypair(&mut OsRng);
        let public_key = PublicKey::from_secp256k1_key(secp_public_key);
        MockSolomachine {
            chain_id: ChainId::from_string(chain_id),
            commitment_prefix,
            public_key,
            secret_key,
            client_states: Arc::new(Mutex::new(HashMap::new())),
            client_consensus_states: Arc::new(Mutex::new(HashMap::new())),
            runtime,
            telemetry,
            connections: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl ChainIdGetter<MockSolomachine> for SolomachineChainComponents {
    fn chain_id(chain: &MockSolomachine) -> &ChainId {
        &chain.chain_id
    }
}

impl IbcCommitmentPrefixGetter<MockSolomachine> for SolomachineChainComponents {
    fn ibc_commitment_prefix(chain: &MockSolomachine) -> &String {
        &chain.commitment_prefix
    }
}

impl<Counterparty> ClientStateQuerier<MockSolomachine, Counterparty> for SolomachineChainComponents
where
    Counterparty: HasClientStateType<MockSolomachine, ClientState = TendermintClientState>,
{
    async fn query_client_state(
        chain: &MockSolomachine,
        client_id: &ClientId,
        _height: &Height,
    ) -> Result<TendermintClientState, Error> {
        let client_states = chain.client_states.lock().unwrap();

        let client_state = client_states
            .get(client_id)
            .ok_or_else(|| eyre!("client state for client id `{}` was not found", client_id))?;

        Ok(client_state.clone())
    }
}

impl<Counterparty> ConsensusStateQuerier<MockSolomachine, Counterparty>
    for SolomachineChainComponents
where
    Counterparty: HasHeightType<Height = Height>
        + HasConsensusStateType<MockSolomachine, ConsensusState = TendermintConsensusState>,
{
    async fn query_consensus_state(
        chain: &MockSolomachine,
        client_id: &ClientId,
        _consensus_height: &Height,
        _query_height: &Height,
    ) -> Result<TendermintConsensusState, Error> {
        let client_consensus_states = chain.client_consensus_states.lock().unwrap();

        let consensus_state = client_consensus_states.get(client_id).ok_or_else(|| {
            eyre!(
                "consensus state for client id `{}` was not found",
                client_id
            )
        })?;

        Ok(consensus_state.clone())
    }
}

impl<Counterparty> ConnectionEndQuerier<MockSolomachine, Counterparty>
    for SolomachineChainComponents
{
    async fn query_connection_end(
        chain: &MockSolomachine,
        connection_id: &ConnectionId,
        _height: &Height,
    ) -> Result<ConnectionEnd, Error> {
        let connections = chain.connections.lock().unwrap();

        let connection = connections.get(connection_id).ok_or_else(|| {
            eyre!(
                "connection end for connection id `{}` was not found",
                connection_id
            )
        })?;

        Ok(connection.clone())
    }
}

impl<Counterparty> ChannelEndQuerier<MockSolomachine, Counterparty> for SolomachineChainComponents {
    async fn query_channel_end(
        _chain: &MockSolomachine,
        _channel_id: &ChannelId,
        _port_id: &PortId,
        _height: &Height,
    ) -> Result<ChannelEnd, Error> {
        todo!()
    }
}

impl Solomachine for MockSolomachine {
    fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    fn current_time(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("error computing current time")
            .as_secs()
    }

    fn current_diversifier(&self) -> String {
        DEFAULT_DIVERSIFIER.to_owned()
    }

    async fn create_client(
        &self,
        client_state: TendermintClientState,
        consensus_state: TendermintConsensusState,
    ) -> Result<ClientId, Self::Error> {
        let client_id = ClientId::from_str("cosmos-client").unwrap();
        {
            let mut client_states = self.client_states.lock().unwrap();
            client_states.insert(client_id.clone(), client_state);
        }
        {
            let mut client_consensus_states: std::sync::MutexGuard<
                '_,
                HashMap<ClientId, TendermintConsensusState>,
            > = self.client_consensus_states.lock().unwrap();
            client_consensus_states.insert(client_id.clone(), consensus_state);
        }
        Ok(client_id)
    }

    async fn update_connection(&self, connection_id: &ConnectionId, connection_end: ConnectionEnd) {
        let mut connections = self.connections.lock().unwrap();
        connections.insert(connection_id.clone(), connection_end);
    }

    async fn handle_receive_packet(&self, _packet: &Packet) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}
