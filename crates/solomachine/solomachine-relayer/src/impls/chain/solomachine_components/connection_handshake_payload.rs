use core::str::FromStr;

use cgp_core::error::{CanRaiseError, HasErrorType};
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilder, ConnectionOpenConfirmPayloadBuilder,
    ConnectionOpenInitPayloadBuilder, ConnectionOpenTryPayloadBuilder,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::connection::types::version::Version;
use ibc::core::connection::types::State as ConnectionState;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::Height;

use crate::methods::proofs::client_state::client_state_proof_data;
use crate::methods::proofs::connection::connection_proof_data;
use crate::methods::proofs::consensus_state::consensus_state_proof_data;
use crate::traits::solomachine::Solomachine;
use crate::types::client_state::SolomachineClientState;
use crate::types::payloads::connection::{
    SolomachineConnectionOpenAckPayload, SolomachineConnectionOpenConfirmPayload,
    SolomachineConnectionOpenInitPayload, SolomachineConnectionOpenTryPayload,
};

pub struct BuildSolomachineConnectionHandshakePayloads;

impl<Chain, Counterparty> ConnectionOpenInitPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakePayloads
where
    Chain: Solomachine
        + HasConnectionOpenInitPayloadType<
            Counterparty,
            ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + HasErrorType,
{
    async fn build_connection_open_init_payload(
        chain: &Chain,
        _client_state: &SolomachineClientState,
    ) -> Result<SolomachineConnectionOpenInitPayload, Chain::Error> {
        let commitment_prefix = chain.commitment_prefix();

        let payload = SolomachineConnectionOpenInitPayload {
            commitment_prefix: commitment_prefix.into(),
        };

        Ok(payload)
    }
}

impl<Chain, Counterparty> ConnectionOpenTryPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakePayloads
where
    Chain: Solomachine
        + HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasConnectionOpenTryPayloadType<
            Counterparty,
            ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + CanRaiseError<String>,
{
    async fn build_connection_open_try_payload(
        chain: &Chain,
        solo_client_state: &SolomachineClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<SolomachineConnectionOpenTryPayload, Chain::Error> {
        let connection = chain.query_connection(connection_id).await?;

        if connection.state != ConnectionState::Init {
            return Err(Chain::raise_error(format!(
                "connection state error, expected {} got {}",
                ConnectionState::Init,
                connection.state
            )));
        }

        let versions = connection.versions().to_vec();

        let delay_period = connection.delay_period();

        let commitment_prefix = chain.commitment_prefix();

        let public_key = chain.public_key();
        let secret_key = chain.secret_key();

        let connection_proof = connection_proof_data(
            public_key,
            secret_key,
            solo_client_state,
            commitment_prefix,
            connection_id,
            connection,
        );

        let cosmos_client_state = chain.query_client_state(client_id).await?;

        let client_state_proof = client_state_proof_data(
            public_key,
            secret_key,
            solo_client_state,
            commitment_prefix,
            client_id,
            &cosmos_client_state,
        );

        let cosmos_consensus_state = chain.query_consensus_state(client_id, *height).await?;

        let consensus_state_proof = consensus_state_proof_data(
            secret_key,
            solo_client_state,
            commitment_prefix,
            client_id,
            *height,
            &cosmos_consensus_state,
        );

        let payload = SolomachineConnectionOpenTryPayload {
            commitment_prefix: commitment_prefix.into(),
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
}

impl<Chain, Counterparty> ConnectionOpenAckPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakePayloads
where
    Chain: Solomachine
        + HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasConnectionOpenAckPayloadType<
            Counterparty,
            ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + CanRaiseError<String>,
{
    async fn build_connection_open_ack_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<SolomachineConnectionOpenAckPayload, Chain::Error> {
        let public_key = chain.public_key();
        let secret_key = chain.secret_key();

        let connection = chain.query_connection(connection_id).await?;

        if connection.state != ConnectionState::TryOpen {
            return Err(Chain::raise_error(format!(
                "connection state error, expected {} got {}",
                ConnectionState::TryOpen,
                connection.state
            )));
        }

        let version = connection
            .versions()
            .iter()
            .next()
            .cloned()
            .or_else(|| Version::compatibles().into_iter().next())
            .unwrap();

        let commitment_prefix = chain.commitment_prefix();

        let cosmos_client_state = chain.query_client_state(client_id).await?;

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

        let cosmos_consensus_state = chain.query_consensus_state(client_id, *height).await?;

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
}

impl<Chain, Counterparty> ConnectionOpenConfirmPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakePayloads
where
    Chain: Solomachine
        + HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + HasConnectionOpenConfirmPayloadType<
            Counterparty,
            ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload,
        > + HasErrorType,
{
    async fn build_connection_open_confirm_payload(
        chain: &Chain,
        client_state: &SolomachineClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<SolomachineConnectionOpenConfirmPayload, Chain::Error> {
        let public_key = chain.public_key();
        let secret_key = chain.secret_key();
        let commitment_prefix = chain.commitment_prefix();
        let _cosmos_client_state = chain.query_client_state(client_id).await?;

        let connection = chain
            .query_connection(&ConnectionId::from_str(connection_id.as_str()).unwrap())
            .await?;

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
