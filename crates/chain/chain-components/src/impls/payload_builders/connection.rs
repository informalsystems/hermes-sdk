use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::connection_id::HasConnectionIdType;

use crate::traits::commitment_prefix::HasIbcCommitmentPrefix;
use crate::traits::payload_builders::connection_handshake::{
    ConnectionOpenAckPayloadBuilder, ConnectionOpenAckPayloadBuilderComponent,
    ConnectionOpenConfirmPayloadBuilder, ConnectionOpenConfirmPayloadBuilderComponent,
    ConnectionOpenInitPayloadBuilder, ConnectionOpenInitPayloadBuilderComponent,
    ConnectionOpenTryPayloadBuilder, ConnectionOpenTryPayloadBuilderComponent,
};
use crate::traits::queries::client_state::CanQueryClientStateWithProofs;
use crate::traits::queries::connection_end::CanQueryConnectionEndWithProofs;
use crate::traits::queries::consensus_state::CanQueryConsensusStateWithProofs;
use crate::traits::types::client_state::{HasClientStateFields, HasClientStateType};
use crate::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};
use crate::traits::types::consensus_state::HasConsensusStateType;
use crate::traits::types::proof::HasCommitmentProofHeight;
use crate::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};

pub struct BuildConnectionHandshakePayload;

#[cgp_provider(ConnectionOpenInitPayloadBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenInitPayloadBuilder<Chain, Counterparty>
    for BuildConnectionHandshakePayload
where
    Chain: HasIbcCommitmentPrefix
        + HasClientStateType<Counterparty>
        + HasConnectionOpenInitPayloadType<
            Counterparty,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Chain>,
        > + HasAsyncErrorType,
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

#[cgp_provider(ConnectionOpenTryPayloadBuilderComponent)]
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
        + HasAsyncErrorType,
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
            .query_client_state_with_proofs(PhantomData, client_id, height)
            .await?;

        let consensus_state_height = Counterparty::client_state_latest_height(&client_state);

        let (_, consensus_state_proofs) = chain
            .query_consensus_state_with_proofs(
                PhantomData,
                client_id,
                &consensus_state_height,
                height,
            )
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

#[cgp_provider(ConnectionOpenAckPayloadBuilderComponent)]
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
        + HasAsyncErrorType,
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
            .query_client_state_with_proofs(PhantomData, client_id, height)
            .await?;

        let consensus_state_height = Counterparty::client_state_latest_height(&client_state);

        let (_, consensus_state_proofs) = chain
            .query_consensus_state_with_proofs(
                PhantomData,
                client_id,
                &consensus_state_height,
                height,
            )
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

#[cgp_provider(ConnectionOpenConfirmPayloadBuilderComponent)]
impl<Chain, Counterparty> ConnectionOpenConfirmPayloadBuilder<Chain, Counterparty>
    for BuildConnectionHandshakePayload
where
    Chain: HasConnectionOpenConfirmPayloadType<
            Counterparty,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Chain>,
        > + HasHeightType
        + HasClientIdType<Counterparty>
        + HasConnectionIdType<Counterparty>
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
