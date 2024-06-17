use cgp_core::HasErrorType;

use crate::chain::traits::commitment_prefix::HasIbcCommitmentPrefix;
use crate::chain::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilder, ConnectionOpenConfirmPayloadBuilder,
    ConnectionOpenInitPayloadBuilder, ConnectionOpenTryPayloadBuilder,
};
use crate::chain::traits::queries::client_state::CanQueryClientStateWithProofs;
use crate::chain::traits::queries::connection_end::CanQueryConnectionEndWithProofs;
use crate::chain::traits::queries::consensus_state::CanQueryConsensusStateWithProofs;
use crate::chain::traits::types::client_state::{HasClientStateFields, HasClientStateType};
use crate::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::proof::HasCommitmentProofHeight;
use crate::chain::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};

pub struct BuildConnectionHandshakePayload;

impl<Chain, Counterparty> ConnectionOpenInitPayloadBuilder<Chain, Counterparty>
    for BuildConnectionHandshakePayload
where
    Chain: HasIbcCommitmentPrefix
        + HasClientStateType<Counterparty>
        + HasConnectionOpenInitPayloadType<
            Counterparty,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Chain>,
        > + HasErrorType,
    Chain::CommitmentPrefix: Clone,
{
    async fn build_connection_open_init_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
    ) -> Result<ConnectionOpenInitPayload<Chain>, Chain::Error> {
        let commitment_prefix = chain.ibc_commitment_prefix().clone();
        Ok(ConnectionOpenInitPayload { commitment_prefix })
    }
}

impl<Chain, Counterparty> ConnectionOpenTryPayloadBuilder<Chain, Counterparty>
    for BuildConnectionHandshakePayload
where
    Chain: HasIbcCommitmentPrefix
        + HasCommitmentProofHeight
        + HasClientStateType<Counterparty>
        + HasConnectionOpenTryPayloadType<
            Counterparty,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Chain, Counterparty>,
        > + CanQueryConnectionEndWithProofs<Counterparty>
        + CanQueryClientStateWithProofs<Counterparty>
        + CanQueryConsensusStateWithProofs<Counterparty>
        + HasErrorType,
    Chain::CommitmentPrefix: Clone,
    Counterparty: HasClientStateFields<Chain> + HasConsensusStateType<Chain>,
{
    async fn build_connection_open_try_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<ConnectionOpenTryPayload<Chain, Counterparty>, Chain::Error> {
        let commitment_prefix = chain.ibc_commitment_prefix().clone();

        let (connection_end, connection_proofs) = chain
            .query_connection_end_with_proofs(connection_id, height)
            .await?;

        let (client_state, client_state_proofs) = chain
            .query_client_state_with_proofs(client_id, height)
            .await?;

        let consensus_state_height = Counterparty::client_state_latest_height(&client_state);

        let (_, consensus_state_proofs) = chain
            .query_consensus_state_with_proofs(client_id, &consensus_state_height, height)
            .await?;

        // TODO: validate client and connection states

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&connection_proofs).clone();

        let payload = ConnectionOpenTryPayload {
            commitment_prefix,
            client_state,
            connection_end,
            update_height,
            proof_init: connection_proofs,
            proof_client: client_state_proofs,
            proof_consensus: consensus_state_proofs,
            proof_consensus_height: consensus_state_height,
        };

        Ok(payload)
    }
}

impl<Chain, Counterparty> ConnectionOpenAckPayloadBuilder<Chain, Counterparty>
    for BuildConnectionHandshakePayload
where
    Chain: HasIbcCommitmentPrefix
        + HasCommitmentProofHeight
        + HasClientStateType<Counterparty>
        + HasConnectionOpenAckPayloadType<
            Counterparty,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload<Chain, Counterparty>,
        > + CanQueryConnectionEndWithProofs<Counterparty>
        + CanQueryClientStateWithProofs<Counterparty>
        + CanQueryConsensusStateWithProofs<Counterparty>
        + HasErrorType,
    Chain::CommitmentPrefix: Clone,
    Counterparty: HasClientStateFields<Chain> + HasConsensusStateType<Chain>,
{
    async fn build_connection_open_ack_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<ConnectionOpenAckPayload<Chain, Counterparty>, Chain::Error> {
        let (connection_end, connection_proofs) = chain
            .query_connection_end_with_proofs(connection_id, height)
            .await?;

        let (client_state, client_state_proofs) = chain
            .query_client_state_with_proofs(client_id, height)
            .await?;

        let consensus_state_height = Counterparty::client_state_latest_height(&client_state);

        let (_, consensus_state_proofs) = chain
            .query_consensus_state_with_proofs(client_id, &consensus_state_height, height)
            .await?;

        // TODO: validate client and connection states

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&connection_proofs).clone();

        let payload = ConnectionOpenAckPayload {
            client_state,
            connection_end,
            update_height,
            proof_try: connection_proofs,
            proof_client: client_state_proofs,
            proof_consensus: consensus_state_proofs,
            proof_consensus_height: consensus_state_height,
        };

        Ok(payload)
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmPayloadBuilder<Chain, Counterparty>
    for BuildConnectionHandshakePayload
where
    Chain: HasConnectionOpenConfirmPayloadType<
            Counterparty,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Chain>,
        > + HasIbcChainTypes<Counterparty>
        + HasClientStateType<Counterparty>
        + HasCommitmentProofHeight
        + CanQueryConnectionEndWithProofs<Counterparty>,
{
    async fn build_connection_open_confirm_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        _client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenConfirmPayload, Chain::Error> {
        let (_, connection_proofs) = chain
            .query_connection_end_with_proofs(connection_id, height)
            .await?;

        // TODO: validate connection state

        // TODO: check that all commitment proof heights are the same
        let update_height = Chain::commitment_proof_height(&connection_proofs).clone();

        let payload = ConnectionOpenConfirmPayload {
            update_height,
            proof_ack: connection_proofs,
        };

        Ok(payload)
    }
}
