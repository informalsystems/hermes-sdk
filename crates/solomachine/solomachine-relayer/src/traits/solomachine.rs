use cgp_core::prelude::*;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use ibc::core::connection::types::{ConnectionEnd, State as ConnectionState};
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, ClientId, ConnectionId, PortId};
use ibc_relayer_types::Height;
use secp256k1::SecretKey;

use crate::methods::encode::public_key::PublicKey;

#[async_trait]
pub trait Solomachine: HasErrorType {
    fn get_telemetry(&self) -> &CosmosTelemetry;

    fn invalid_connection_state_error(
        expected: ConnectionState,
        actual: ConnectionState,
    ) -> Self::Error;

    fn public_key(&self) -> &PublicKey;

    // TODO: remove secret key accessor and provide sign methods instead.
    // Doing so would allow multisig or have the private key stored outside
    // of the process, such as in HSM.
    fn secret_key(&self) -> &SecretKey;

    fn commitment_prefix(&self) -> &str;

    fn current_time(&self) -> u64;

    fn current_diversifier(&self) -> String;

    async fn create_client(
        &self,
        client_state: TendermintClientState,
        consensus_state: TendermintConsensusState,
    ) -> Result<ClientId, Self::Error>;

    async fn query_client_state(
        &self,
        client_id: &ClientId,
    ) -> Result<TendermintClientState, Self::Error>;

    async fn query_consensus_state(
        &self,
        client_id: &ClientId,
        height: Height,
    ) -> Result<TendermintConsensusState, Self::Error>;

    async fn update_connection(&self, connection_id: &ConnectionId, connection_end: ConnectionEnd);

    async fn query_connection(
        &self,
        connection_id: &ConnectionId,
    ) -> Result<ConnectionEnd, Self::Error>;

    async fn query_channel(
        &self,
        channel_id: &ChannelId,
        port_id: &PortId,
    ) -> Result<ChannelEnd, Self::Error>;

    async fn handle_receive_packet(&self, packet: &Packet) -> Result<Vec<u8>, Self::Error>;
}
