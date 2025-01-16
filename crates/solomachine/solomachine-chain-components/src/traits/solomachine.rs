use cgp::prelude::*;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use ibc::core::channel::types::packet::Packet;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};
use secp256k1::SecretKey;

use crate::methods::encode::public_key::PublicKey;

#[async_trait]
pub trait Solomachine: HasAsyncErrorType {
    fn public_key(&self) -> &PublicKey;

    // TODO: remove secret key accessor and provide sign methods instead.
    // Doing so would allow multisig or have the private key stored outside
    // of the process, such as in HSM.
    fn secret_key(&self) -> &SecretKey;

    fn current_time(&self) -> u64;

    fn current_diversifier(&self) -> String;

    async fn create_client(
        &self,
        client_state: TendermintClientState,
        consensus_state: TendermintConsensusState,
    ) -> Result<ClientId, Self::Error>;
    async fn update_connection(&self, connection_id: &ConnectionId, connection_end: ConnectionEnd);

    async fn handle_receive_packet(&self, packet: &Packet) -> Result<Vec<u8>, Self::Error>;
}
