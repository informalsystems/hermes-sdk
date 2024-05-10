use cgp_core::prelude::*;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloadTypes, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::proofs::ConsensusProof;

use crate::methods::encode::sign_data::timestamped_sign_data_to_bytes;
use crate::types::payloads::connection::{
    SolomachineConnectionOpenAckPayload, SolomachineConnectionOpenConfirmPayload,
    SolomachineConnectionOpenInitPayload, SolomachineConnectionOpenTryPayload,
};

pub struct BuildSolomachineConnectionHandshakeMessagesForCosmos;

#[async_trait]
impl<Chain, Counterparty> ConnectionHandshakeMessageBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakeMessagesForCosmos
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = CosmosMessage,
        > + HasErrorType<Error = Error>,
    Counterparty: HasConnectionHandshakePayloadTypes<
            Chain,
            ConnectionOpenInitPayload = SolomachineConnectionOpenInitPayload,
            ConnectionOpenTryPayload = SolomachineConnectionOpenTryPayload,
            ConnectionOpenAckPayload = SolomachineConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = SolomachineConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        _client_id: &ClientId,
        _counterparty_client_id: &ClientId,
        _init_connection_options: &Chain::InitConnectionOptions,
        _counterparty_payload: SolomachineConnectionOpenInitPayload,
    ) -> Result<CosmosMessage, Error> {
        todo!()
    }

    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        counterparty_connection_id: &ConnectionId,
        payload: SolomachineConnectionOpenTryPayload,
    ) -> Result<CosmosMessage, Error> {
        let counterparty_commitment_prefix = payload.commitment_prefix;

        let proof_init: ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes = timestamped_sign_data_to_bytes(&payload.proof_init)
            .try_into()?;

        let proof_client = timestamped_sign_data_to_bytes(&payload.proof_client).try_into()?;

        let consensus_signature =
            timestamped_sign_data_to_bytes(&payload.proof_consensus).try_into()?;

        let proof_consensus = ConsensusProof::new(consensus_signature, payload.update_height)?;

        let message = CosmosConnectionOpenTryMessage {
            client_id: client_id.clone(),
            counterparty_client_id: counterparty_client_id.clone(),
            counterparty_connection_id: counterparty_connection_id.clone(),
            counterparty_commitment_prefix,
            counterparty_versions: payload.versions,
            delay_period: payload.delay_period,
            client_state: payload.client_state.into(),
            update_height: payload.update_height,
            proof_init,
            proof_client,
            proof_consensus,
        };

        Ok(message.to_cosmos_message())
    }

    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_connection_id: &ConnectionId,
        _counterparty_payload: SolomachineConnectionOpenAckPayload,
    ) -> Result<CosmosMessage, Error> {
        todo!()
    }

    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_payload: SolomachineConnectionOpenConfirmPayload,
    ) -> Result<CosmosMessage, Error> {
        todo!()
    }
}
