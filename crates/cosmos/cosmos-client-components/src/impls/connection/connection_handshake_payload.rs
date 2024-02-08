use cgp_core::CanRaiseError;
use eyre::eyre;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::ConnectionHandshakePayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionHandshakePayloadTypes;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{IncludeProof, QueryConnectionRequest, QueryHeight};
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer::connection::ConnectionMsgType;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};

pub struct BuildCosmosConnectionHandshakePayload;

impl<Chain, Counterparty> ConnectionHandshakePayloadBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakePayload
where
    Chain: HasConnectionHandshakePayloadTypes<
            Counterparty,
            ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload,
            ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload,
            ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload,
        > + HasClientStateType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Height = Height,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasBlockingChainHandle
        + CanRaiseError<eyre::Report>,
{
    async fn build_connection_open_init_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
    ) -> Result<Chain::ConnectionOpenInitPayload, Chain::Error> {
        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let commitment_prefix = chain_handle
                    .query_commitment_prefix()
                    .map_err(Chain::raise_error)?;

                Ok(CosmosConnectionOpenInitPayload { commitment_prefix })
            })
            .await
    }

    async fn build_connection_open_try_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenTryPayload, Chain::Error> {
        let height = *height;
        let client_id = client_id.clone();
        let connection_id = connection_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let commitment_prefix = chain_handle
                    .query_commitment_prefix()
                    .map_err(Chain::raise_error)?;

                let (connection, _) = chain_handle
                    .query_connection(
                        QueryConnectionRequest {
                            connection_id: connection_id.clone(),
                            height: QueryHeight::Latest,
                        },
                        IncludeProof::No,
                    )
                    .map_err(Chain::raise_error)?;

                let versions = connection.versions().to_vec();
                let delay_period = connection.delay_period();

                let (m_client_state, proofs) = chain_handle
                    .build_connection_proofs_and_client_state(
                        ConnectionMsgType::OpenTry,
                        &connection_id,
                        &client_id,
                        height,
                    )
                    .map_err(Chain::raise_error)?;

                let any_client_state = m_client_state
                    .ok_or_else(|| Chain::raise_error(eyre!("expect some client state")))?;

                let client_state = match any_client_state {
                    AnyClientState::Tendermint(client_state) => client_state,
                };

                let proof_client = proofs
                    .client_proof()
                    .ok_or_else(|| Chain::raise_error(eyre!("expect non empty client proof")))?
                    .clone();

                let proof_consensus = proofs
                    .consensus_proof()
                    .ok_or_else(|| Chain::raise_error(eyre!("expect non empty consensus proof")))?
                    .clone();

                let payload = CosmosConnectionOpenTryPayload {
                    commitment_prefix,
                    client_state,
                    versions,
                    delay_period,
                    update_height: proofs.height(),
                    proof_init: proofs.object_proof().clone(),
                    proof_client,
                    proof_consensus,
                };

                Ok(payload)
            })
            .await
    }

    async fn build_connection_open_ack_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenAckPayload, Chain::Error> {
        let height = *height;
        let client_id = client_id.clone();
        let connection_id = connection_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (connection, _) = chain_handle
                    .query_connection(
                        QueryConnectionRequest {
                            connection_id: connection_id.clone(),
                            height: QueryHeight::Latest,
                        },
                        IncludeProof::No,
                    )
                    .map_err(Chain::raise_error)?;

                let version = connection
                    .versions()
                    .iter()
                    .next()
                    .cloned()
                    .unwrap_or_default();

                let (m_client_state, proofs) = chain_handle
                    .build_connection_proofs_and_client_state(
                        ConnectionMsgType::OpenAck,
                        &connection_id,
                        &client_id,
                        height,
                    )
                    .map_err(Chain::raise_error)?;

                let any_client_state = m_client_state
                    .ok_or_else(|| Chain::raise_error(eyre!("expect some client state")))?;

                let client_state = match any_client_state {
                    AnyClientState::Tendermint(client_state) => client_state,
                };

                let proof_client = proofs
                    .client_proof()
                    .ok_or_else(|| Chain::raise_error(eyre!("expect non empty client proof")))?
                    .clone();

                let proof_consensus = proofs
                    .consensus_proof()
                    .ok_or_else(|| Chain::raise_error(eyre!("expect non empty consensus proof")))?
                    .clone();

                let payload = CosmosConnectionOpenAckPayload {
                    client_state,
                    version,
                    update_height: proofs.height(),
                    proof_try: proofs.object_proof().clone(),
                    proof_client,
                    proof_consensus,
                };

                Ok(payload)
            })
            .await
    }

    async fn build_connection_open_confirm_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Chain::Height,
        client_id: &Chain::ClientId,
        connection_id: &Chain::ConnectionId,
    ) -> Result<Chain::ConnectionOpenConfirmPayload, Chain::Error> {
        let height = *height;
        let client_id = client_id.clone();
        let connection_id = connection_id.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let (_, proofs) = chain_handle
                    .build_connection_proofs_and_client_state(
                        ConnectionMsgType::OpenConfirm,
                        &connection_id,
                        &client_id,
                        height,
                    )
                    .map_err(Chain::raise_error)?;

                let update_height = proofs.height();
                let proof_ack = proofs.object_proof().clone();

                let payload = CosmosConnectionOpenConfirmPayload {
                    update_height,
                    proof_ack,
                };

                Ok(payload)
            })
            .await
    }
}
