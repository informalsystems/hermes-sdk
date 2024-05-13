use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionOpenAckMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionOpenConfirmMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionOpenInitMessageBuilder;
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::ConnectionOpenTryMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionOpenAckPayloadType;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionOpenConfirmPayloadType;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionOpenInitPayloadType;
use hermes_relayer_components::chain::traits::types::connection::HasConnectionOpenTryPayloadType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

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
            ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_init_message(
        _chain: &Chain,
        _client_id: &ClientId,
        _counterparty_client_id: &ClientId,
        _init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: CosmosConnectionOpenInitPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosConnectionOpenInit(Box::new(counterparty_payload));

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
        > + HasErrorType,
    Counterparty: HasConnectionOpenTryPayloadType<
            Chain,
            ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_try_message(
        _chain: &Chain,
        _client_id: &ClientId,
        _counterparty_client_id: &ClientId,
        _counterparty_connection_id: &ConnectionId,
        counterparty_payload: CosmosConnectionOpenTryPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosConnectionOpenTry(Box::new(counterparty_payload));

        Ok(message)
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
        > + HasErrorType,
    Counterparty: HasConnectionOpenAckPayloadType<
            Chain,
            ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_connection_id: &ConnectionId,
        counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosConnectionOpenAck(Box::new(counterparty_payload));

        Ok(message)
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
    Counterparty: HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>,
{
    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        counterparty_payload: CosmosConnectionOpenConfirmPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message =
            SolomachineMessage::CosmosConnectionOpenConfirm(Box::new(counterparty_payload));

        Ok(message)
    }
}
