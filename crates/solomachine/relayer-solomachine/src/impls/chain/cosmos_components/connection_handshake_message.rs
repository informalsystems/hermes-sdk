use alloc::sync::Arc;
use async_trait::async_trait;
use cgp_core::{DelegateComponent, HasErrorType};
use cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use cosmos_client_components::types::messages::connection::open_try::CosmosConnectionOpenTryMessage;
use hermes_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloads, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_cosmos::impls::chain::components::connection_handshake_message::DelegateCosmosConnectionHandshakeBuilder;
use ibc_relayer_cosmos::types::error::{BaseError, Error};
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};
use ibc_relayer_types::proofs::ConsensusProof;

use crate::methods::encode::sign_data::timestamped_sign_data_to_bytes;
use crate::traits::solomachine::Solomachine;
use crate::types::chain::SolomachineChain;
use crate::types::payloads::connection::{
    SolomachineConnectionOpenAckPayload, SolomachineConnectionOpenConfirmPayload,
    SolomachineConnectionOpenInitPayload, SolomachineConnectionOpenTryPayload,
};

pub struct BuildSolomachineConnectionHandshakeMessagesForCosmos;

impl<Counterparty> DelegateComponent<SolomachineChain<Counterparty>>
    for DelegateCosmosConnectionHandshakeBuilder
where
    Counterparty: Solomachine,
{
    type Delegate = BuildSolomachineConnectionHandshakeMessagesForCosmos;
}

#[async_trait]
impl<Chain, Counterparty> ConnectionHandshakeMessageBuilder<Chain, Counterparty>
    for BuildSolomachineConnectionHandshakeMessagesForCosmos
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            Message = Arc<dyn CosmosMessage>,
        > + HasErrorType<Error = Error>,
    Counterparty: HasConnectionHandshakePayloads<
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
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        todo!()
    }

    async fn build_connection_open_try_message(
        _chain: &Chain,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        counterparty_connection_id: &ConnectionId,
        payload: SolomachineConnectionOpenTryPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        let counterparty_commitment_prefix = Vec::from(payload.commitment_prefix)
            .try_into()
            .map_err(BaseError::ics23)?;

        let proof_init: ibc_relayer_types::core::ics23_commitment::commitment::CommitmentProofBytes = timestamped_sign_data_to_bytes(&payload.proof_init).unwrap()
            .try_into()
            .map_err(BaseError::proofs)?;

        let proof_client = timestamped_sign_data_to_bytes(&payload.proof_client)
            .unwrap()
            .try_into()
            .map_err(BaseError::proofs)?;

        let consensus_signature = timestamped_sign_data_to_bytes(&payload.proof_consensus)
            .unwrap()
            .try_into()
            .map_err(BaseError::proofs)?;

        let proof_consensus = ConsensusProof::new(consensus_signature, payload.update_height)
            .map_err(BaseError::proofs)?;

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
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        todo!()
    }

    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_payload: SolomachineConnectionOpenConfirmPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        todo!()
    }
}
