use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_cosmos_chain_components::types::tendermint::{
    TendermintClientState, TendermintConsensusState,
};
use hermes_relayer_components::chain::traits::commitment_prefix::HasIbcCommitmentPrefix;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilder, ConnectionOpenAckPayloadBuilderComponent,
    ConnectionOpenConfirmPayloadBuilder, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilder, ConnectionOpenInitPayloadBuilderComponent,
    ConnectionOpenTryPayloadBuilder, ConnectionOpenTryPayloadBuilderComponent,
};
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::queries::connection_end::CanQueryConnectionEnd;
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::client::types::Height;
use ibc::core::connection::types::version::Version;
use ibc::core::connection::types::{ConnectionEnd, State as ConnectionState};
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};

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

#[cgp_provider(ConnectionOpenInitPayloadBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenInitPayloadBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakePayloads
where
    Chain: Solomachine
        + HasConnectionOpenInitPayloadType<
            Counterparty,
            ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload,
        > + HasClientStateType<Counterparty, ClientState = SolomachineClientState>
        + HasIbcCommitmentPrefix<CommitmentPrefix = String>
        + HasAsyncErrorType,
{
    async fn build_connection_open_init_payload(
        chain: &Chain,
        _client_state: &SolomachineClientState,
    ) -> Result<SolomachineConnectionOpenInitPayload, Chain::Error> {
        let commitment_prefix = chain.ibc_commitment_prefix();

        let payload = SolomachineConnectionOpenInitPayload {
            commitment_prefix: commitment_prefix.as_bytes().into(),
        };

        Ok(payload)
    }
}

#[cgp_provider(ConnectionOpenTryPayloadBuilderComponent)]
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
        + CanQueryClientState<Counterparty>
        + CanQueryConsensusState<Counterparty>
        + CanQueryConnectionEnd<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcCommitmentPrefix<CommitmentPrefix = String>
        + CanRaiseAsyncError<String>,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>
        + HasConsensusStateType<Chain, ConsensusState = TendermintConsensusState>
        + HasHeightType<Height = Height>,
{
    async fn build_connection_open_try_payload(
        chain: &Chain,
        solo_client_state: &SolomachineClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<SolomachineConnectionOpenTryPayload, Chain::Error> {
        let connection = chain.query_connection_end(connection_id, height).await?;

        if connection.state != ConnectionState::Init {
            return Err(Chain::raise_error(format!(
                "connection state error, expected {} got {}",
                ConnectionState::Init,
                connection.state
            )));
        }

        let versions = connection.versions().to_vec();

        let delay_period = connection.delay_period();

        let commitment_prefix = chain.ibc_commitment_prefix();

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

        let cosmos_client_state = chain
            .query_client_state(PhantomData, client_id, height)
            .await?;

        let client_state_proof = client_state_proof_data(
            public_key,
            secret_key,
            solo_client_state,
            commitment_prefix,
            client_id,
            &cosmos_client_state,
        );

        let cosmos_consensus_state = chain
            .query_consensus_state(PhantomData, client_id, height, height)
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
            commitment_prefix: commitment_prefix.as_bytes().into(),
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

#[cgp_provider(ConnectionOpenAckPayloadBuilderComponent)]
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
        + CanQueryClientState<Counterparty>
        + CanQueryConsensusState<Counterparty>
        + CanQueryConnectionEnd<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcCommitmentPrefix<CommitmentPrefix = String>
        + CanRaiseAsyncError<String>,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>
        + HasConsensusStateType<Chain, ConsensusState = TendermintConsensusState>
        + HasHeightType<Height = Height>,
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

        let connection = chain.query_connection_end(connection_id, height).await?;

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

        let commitment_prefix = chain.ibc_commitment_prefix();

        let cosmos_client_state = chain
            .query_client_state(PhantomData, client_id, height)
            .await?;

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
            .query_consensus_state(PhantomData, client_id, height, height)
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
}

#[cgp_provider(ConnectionOpenConfirmPayloadBuilderComponent)]
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
        > + CanQueryClientState<Counterparty>
        + CanQueryConnectionEnd<Counterparty, ConnectionEnd = ConnectionEnd>
        + HasIbcCommitmentPrefix<CommitmentPrefix = String>
        + HasAsyncErrorType,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>,
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
        let commitment_prefix = chain.ibc_commitment_prefix();
        let _cosmos_client_state = chain
            .query_client_state(PhantomData, client_id, height)
            .await?;

        let connection = chain.query_connection_end(connection_id, height).await?;

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
