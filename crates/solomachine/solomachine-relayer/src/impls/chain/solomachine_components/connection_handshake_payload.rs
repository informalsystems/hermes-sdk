use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilder;
use ibc_relayer_types::core::ics03_connection::connection::State as ConnectionState;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::Height;

use crate::methods::proofs::client_state::client_state_proof_data;
use crate::methods::proofs::connection::connection_proof_data;
use crate::methods::proofs::consensus_state::consensus_state_proof_data;
use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::client_state::SolomachineClientState;
use crate::types::payloads::connection::{
    SolomachineConnectionOpenAckPayload, SolomachineConnectionOpenConfirmPayload,
    SolomachineConnectionOpenInitPayload, SolomachineConnectionOpenTryPayload,
};

pub struct BuildSolomachineConnectionHandshakePayloads;

#[async_trait]
impl<Chain, Counterparty> ConnectionHandshakePayloadBuilder<SolomachineChain<Chain>, Counterparty>
    for BuildSolomachineConnectionHandshakePayloads
where
    Chain: Solomachine,
{
    async fn build_connection_open_init_payload(
        chain: &SolomachineChain<Chain>,
        _client_state: &SolomachineClientState,
    ) -> Result<SolomachineConnectionOpenInitPayload, Chain::Error> {
        let commitment_prefix = chain.chain.commitment_prefix();

        let payload = SolomachineConnectionOpenInitPayload {
            commitment_prefix: commitment_prefix.to_string(),
        };

        Ok(payload)
    }

    async fn build_connection_open_try_payload(
        chain: &SolomachineChain<Chain>,
        solo_client_state: &SolomachineClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<SolomachineConnectionOpenTryPayload, Chain::Error> {
        let connection = chain.chain.query_connection(connection_id).await?;

        if connection.state != ConnectionState::Init {
            return Err(Chain::invalid_connection_state_error(
                ConnectionState::Init,
                connection.state,
            ));
        }

        let versions = connection.versions().to_vec();

        let delay_period = connection.delay_period();

        let commitment_prefix = chain.chain.commitment_prefix();

        let public_key = chain.chain.public_key();
        let secret_key = chain.chain.secret_key();

        let connection_proof = connection_proof_data(
            public_key,
            secret_key,
            solo_client_state,
            commitment_prefix,
            connection_id,
            connection,
        );

        let cosmos_client_state = chain.chain.query_client_state(client_id).await?;

        let client_state_proof = client_state_proof_data(
            public_key,
            secret_key,
            solo_client_state,
            commitment_prefix,
            client_id,
            &cosmos_client_state,
        );

        let cosmos_consensus_state = chain
            .chain
            .query_consensus_state(client_id, *height)
            .await?;

        let consensus_state_proof = consensus_state_proof_data(
            secret_key,
            solo_client_state,
            commitment_prefix,
            client_id,
            *height,
            &cosmos_consensus_state,
        );

        let payload = SolomachineConnectionOpenTryPayload {
            commitment_prefix: commitment_prefix.to_string(),
            client_state: cosmos_client_state,
            versions,
            delay_period,
            update_height: *height,
            proof_init: connection_proof,
            proof_client: client_state_proof,
            proof_consensus: consensus_state_proof,
        };

        Ok(payload)
    }

    async fn build_connection_open_ack_payload(
        chain: &SolomachineChain<Chain>,
        client_state: &SolomachineClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<SolomachineConnectionOpenAckPayload, Chain::Error> {
        let public_key = chain.chain.public_key();
        let secret_key = chain.chain.secret_key();
        let connection = chain.chain.query_connection(connection_id).await?;

        if connection.state != ConnectionState::TryOpen {
            return Err(Chain::invalid_connection_state_error(
                ConnectionState::TryOpen,
                connection.state,
            ));
        }

        let version = connection
            .versions()
            .iter()
            .next()
            .cloned()
            .unwrap_or_default();

        let commitment_prefix = chain.chain.commitment_prefix();

        let cosmos_client_state = chain.chain.query_client_state(client_id).await?;

        let client_state_proof = client_state_proof_data(
            public_key,
            secret_key,
            client_state,
            commitment_prefix,
            client_id,
            &cosmos_client_state,
        );

        let connection_proof: crate::types::sign_data::SolomachineTimestampedSignData =
            connection_proof_data(
                public_key,
                secret_key,
                client_state,
                commitment_prefix,
                connection_id,
                connection,
            );

        let cosmos_consensus_state = chain
            .chain
            .query_consensus_state(client_id, *height)
            .await?;

        let consensus_state_proof = consensus_state_proof_data(
            secret_key,
            client_state,
            commitment_prefix,
            client_id,
            *height,
            &cosmos_consensus_state,
        );

        let payload = SolomachineConnectionOpenAckPayload {
            client_state: cosmos_client_state,
            version,
            update_height: *height,
            proof_try: connection_proof,
            proof_client: client_state_proof,
            proof_consensus: consensus_state_proof,
        };

        Ok(payload)
    }

    async fn build_connection_open_confirm_payload(
        chain: &SolomachineChain<Chain>,
        client_state: &SolomachineClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<SolomachineConnectionOpenConfirmPayload, Chain::Error> {
        let public_key = chain.chain.public_key();
        let secret_key = chain.chain.secret_key();
        let commitment_prefix = chain.chain.commitment_prefix();
        let _cosmos_client_state = chain.chain.query_client_state(client_id).await?;

        let connection = chain.chain.query_connection(connection_id).await?;

        // TODO confirm connection state
        /*if connection.state != ConnectionState::TryOpen {
            return Err(Chain::invalid_connection_state_error(
                ConnectionState::TryOpen,
                connection.state,
            ));
        }*/

        let connection_proof: crate::types::sign_data::SolomachineTimestampedSignData =
            connection_proof_data(
                public_key,
                secret_key,
                client_state,
                commitment_prefix,
                connection_id,
                connection,
            );

        let payload = SolomachineConnectionOpenConfirmPayload {
            update_height: *height,
            proof_ack: connection_proof,
        };

        Ok(payload)
    }
}
