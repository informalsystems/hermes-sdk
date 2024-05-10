use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_relayer_components::chain::traits::commitment_prefix::HasIbcCommitmentPrefix;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilder;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStateWithProofs;
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryRawConsensusStateWithProofs;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::connection::HasConnectionHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::height::{
    CanIncrementHeight, HasHeightFields,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_relayer_types::core::ics02_client::error::Error as Ics02Error;
use ibc_relayer_types::core::ics03_connection::connection::ConnectionEnd;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::Height;
use prost_types::Any;

use crate::traits::grpc_address::HasGrpcAddress;
use crate::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};

pub struct BuildCosmosConnectionHandshakePayload;

impl<Chain, Counterparty, Encoding> ConnectionHandshakePayloadBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakePayload
where
    Chain: HasConnectionHandshakePayloadTypes<
            Counterparty,
            ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload,
            ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload,
            ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty>
        + HasIbcCommitmentPrefix<CommitmentPrefix = Vec<u8>>
        + HasCommitmentProofType<CommitmentProof = Vec<u8>>
        + CanIncrementHeight
        + CanQueryConnectionEnd<Counterparty, ConnectionEnd = ConnectionEnd>
        + CanQueryConnectionEndWithProofs<Counterparty, ConnectionEnd = ConnectionEnd>
        + CanQueryClientStateWithProofs<Counterparty>
        + CanQueryRawConsensusStateWithProofs<Counterparty, RawConsensusState = Any>
        + HasGrpcAddress
        + CanRaiseError<Encoding::Error>
        + CanRaiseError<Ics02Error>
        + CanRaiseError<&'static str>,
    Counterparty:
        HasClientStateFields<Chain> + HasDefaultEncoding<Encoding = Encoding> + HasHeightFields,
    Encoding: CanConvert<Counterparty::ClientState, Any>,
{
    async fn build_connection_open_init_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
    ) -> Result<Chain::ConnectionOpenInitPayload, Chain::Error> {
        let commitment_prefix = chain.ibc_commitment_prefix().clone();
        Ok(CosmosConnectionOpenInitPayload { commitment_prefix })
    }

    async fn build_connection_open_try_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Height,
        client_id: &ClientId,
        connection_id: &ConnectionId,
    ) -> Result<CosmosConnectionOpenTryPayload, Chain::Error> {
        let commitment_prefix = chain.ibc_commitment_prefix().clone();

        let (connection, connection_proofs) = chain
            .query_connection_end_with_proofs(connection_id, height)
            .await?;

        let versions = connection.versions().to_vec();
        let delay_period = connection.delay_period();

        let (client_state, client_state_proofs) = chain
            .query_client_state_with_proofs(client_id, height)
            .await?;

        let client_state_any = Counterparty::default_encoding()
            .convert(&client_state)
            .map_err(Chain::raise_error)?;

        let consensus_state_height = Counterparty::client_state_latest_height(&client_state);

        let (_, consensus_state_proofs) = chain
            .query_raw_consensus_state_with_proofs(client_id, &consensus_state_height, height)
            .await?;

        let update_height = Chain::increment_height(height)?;

        let proof_consensus_height = Height::new(
            Counterparty::revision_number(&consensus_state_height),
            Counterparty::revision_height(&consensus_state_height),
        )
        .map_err(Chain::raise_error)?;

        let payload = CosmosConnectionOpenTryPayload {
            commitment_prefix,
            client_state: client_state_any,
            versions,
            delay_period,
            update_height,
            proof_init: connection_proofs,
            proof_client: client_state_proofs,
            proof_consensus: consensus_state_proofs,
            proof_consensus_height,
        };

        Ok(payload)
    }

    async fn build_connection_open_ack_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenAckPayload, Chain::Error> {
        let (connection, connection_proofs) = chain
            .query_connection_end_with_proofs(connection_id, height)
            .await?;

        let version = connection
            .versions()
            .iter()
            .next()
            .cloned()
            .unwrap_or_default();

        let (client_state, client_state_proofs) = chain
            .query_client_state_with_proofs(client_id, height)
            .await?;

        let client_state_any = Counterparty::default_encoding()
            .convert(&client_state)
            .map_err(Chain::raise_error)?;

        let consensus_state_height = Counterparty::client_state_latest_height(&client_state);

        let (_, consensus_state_proofs) = chain
            .query_raw_consensus_state_with_proofs(client_id, &consensus_state_height, height)
            .await?;

        let update_height = Chain::increment_height(height)?;

        let proof_consensus_height = Height::new(
            Counterparty::revision_number(&consensus_state_height),
            Counterparty::revision_height(&consensus_state_height),
        )
        .map_err(Chain::raise_error)?;

        let payload = CosmosConnectionOpenAckPayload {
            client_state: client_state_any,
            version,
            update_height,
            proof_try: connection_proofs,
            proof_client: client_state_proofs,
            proof_consensus: consensus_state_proofs,
            proof_consensus_height,
        };

        Ok(payload)
    }

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

        let update_height = Chain::increment_height(height)?;

        let payload = CosmosConnectionOpenConfirmPayload {
            update_height,
            proof_ack: connection_proofs,
        };

        Ok(payload)
    }
}
