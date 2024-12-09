use cgp::core::error::HasErrorType;
use hermes_relayer_components::chain::traits::commitment_prefix::HasCommitmentPrefixType;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionEndType, HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::types::payloads::connection::{
    ConnectionOpenAckPayload, ConnectionOpenConfirmPayload, ConnectionOpenInitPayload,
    ConnectionOpenTryPayload,
};
use ibc::core::host::types::identifiers::{ClientId, ConnectionId};

use crate::types::message::SolomachineMessage;

pub struct BuildCosmosToSolomachineConnectionHandshakeMessage;

impl<Chain, Counterparty> ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineConnectionHandshakeMessage
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasErrorType,
    Counterparty: HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload<Counterparty>,
        > + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
        + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        _client_id: &ClientId,
        _counterparty_client_id: &ClientId,
        _init_connection_options: &Chain::InitConnectionOptions,
        payload: ConnectionOpenInitPayload<Counterparty>,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosConnectionOpenInit {
            commitment_prefix: payload.commitment_prefix,
        };

        Ok(message)
    }
}

impl<Chain, Counterparty> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty>
        + HasErrorType,
    Counterparty: HasCommitmentPrefixType
        + HasCommitmentProofType
        + HasConnectionEndType<Chain>
        + HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload<Counterparty, Chain>,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_try_message(
        _chain: &Chain,
        _client_id: &ClientId,
        _counterparty_client_id: &ClientId,
        _counterparty_connection_id: &ConnectionId,
        _counterparty_payload: ConnectionOpenTryPayload<Counterparty, Chain>,
    ) -> Result<SolomachineMessage, Chain::Error> {
        todo!()
        // let message = SolomachineMessage::CosmosConnectionOpenTry(Box::new(counterparty_payload));

        // Ok(message)
    }
}

impl<Chain, Counterparty> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasClientStateType<Counterparty>
        + HasErrorType,
    Counterparty: HasCommitmentProofType
        + HasConnectionEndType<Chain>
        + HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload<Counterparty, Chain>,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_connection_id: &ConnectionId,
        _counterparty_payload: ConnectionOpenAckPayload<Counterparty, Chain>,
    ) -> Result<SolomachineMessage, Chain::Error> {
        todo!()
        // let message = SolomachineMessage::CosmosConnectionOpenAck(Box::new(counterparty_payload));

        // Ok(message)
    }
}

impl<Chain, Counterparty> ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineConnectionHandshakeMessage
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasErrorType,
    Counterparty: HasCommitmentProofType
        + HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload<Counterparty>,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_payload: ConnectionOpenConfirmPayload<Counterparty>,
    ) -> Result<SolomachineMessage, Chain::Error> {
        todo!()
        // let message =
        //     SolomachineMessage::CosmosConnectionOpenConfirm(Box::new(counterparty_payload));

        // Ok(message)
    }
}
