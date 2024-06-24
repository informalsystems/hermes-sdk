use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use cgp_core::prelude::*;
use eyre::eyre;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_relayer::types::error::Error;
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use hermes_runtime::types::error::TokioRuntimeError;
use hermes_runtime::types::runtime::HermesRuntime;
use ibc::core::connection::types::{ConnectionEnd, State as ConnectionState};
use ibc::core::host::types::identifiers::ConnectionId;
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, ClientId, PortId};
use ibc_relayer_types::Height;
use prost::EncodeError;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Secp256k1, SecretKey};

use crate::methods::encode::public_key::PublicKey;
use crate::traits::solomachine::Solomachine;

const DEFAULT_DIVERSIFIER: &str = "solo-machine-diversifier";

#[derive(Clone)]
pub struct MockSolomachine {
    pub chain_id: ChainId,
    commitment_prefix: String,
    public_key: PublicKey,
    secret_key: SecretKey,
    client_states: Arc<Mutex<HashMap<ClientId, TendermintClientState>>>,
    client_consensus_states: Arc<Mutex<HashMap<ClientId, TendermintConsensusState>>>,
    pub runtime: HermesRuntime,
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

#[async_trait]
impl Solomachine for MockSolomachine {
    type Error = Error;

    fn get_chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    fn get_telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }

    fn runtime(&self) -> &HermesRuntime {
        &self.runtime
    }

    fn runtime_error(e: TokioRuntimeError) -> Self::Error {
        e.into()
    }

    fn encode_error(e: EncodeError) -> Self::Error {
        e.into()
    }

    fn invalid_connection_state_error(
        expected: ConnectionState,
        actual: ConnectionState,
    ) -> Self::Error {
        eyre!(
            "connection state error, expected {} got {}",
            expected,
            actual
        )
        .into()
    }

    fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    fn commitment_prefix(&self) -> &str {
        self.commitment_prefix.as_str()
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

    async fn query_client_state(
        &self,
        client_id: &ClientId,
    ) -> Result<TendermintClientState, Self::Error> {
        let client_states = self.client_states.lock().unwrap();
        client_states
            .get(client_id)
            .ok_or_else(|| eyre!("client state for client id `{}` was not found", client_id).into())
            .cloned()
    }

    async fn query_consensus_state(
        &self,
        client_id: &ClientId,
        _height: Height,
    ) -> Result<TendermintConsensusState, Self::Error> {
        let client_consensus_states = self.client_consensus_states.lock().unwrap();
        client_consensus_states
            .get(client_id)
            .ok_or_else(|| {
                eyre!(
                    "consensus state for client id `{}` was not found",
                    client_id
                )
                .into()
            })
            .cloned()
    }

    async fn update_connection(&self, connection_id: &ConnectionId, connection_end: ConnectionEnd) {
        let mut connections = self.connections.lock().unwrap();
        connections.insert(connection_id.clone(), connection_end);
    }

    async fn query_connection(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<ConnectionEnd, Self::Error> {
        let connections = self.connections.lock().unwrap();
        connections
            .get(connection_id)
            .ok_or_else(|| {
                eyre!(
                    "connection end for connection id `{}` was not found",
                    connection_id
                )
                .into()
            })
            .cloned()
    }

    async fn query_channel(
        &self,
        _channel_id: &ChannelId,
        _port_id: &PortId,
    ) -> Result<ChannelEnd, Self::Error> {
        todo!()
    }

    async fn handle_receive_packet(&self, _packet: &Packet) -> Result<Vec<u8>, Self::Error> {
        todo!()
    }
}
