use cgp_core::prelude::*;
use cgp_core::HasErrorType;
use hermes_cosmos_client_components::types::payloads::connection::{
    CosmosConnectionOpenAckPayload, CosmosConnectionOpenConfirmPayload,
    CosmosConnectionOpenInitPayload, CosmosConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::components::connection_handshake_message_builder::ConnectionHandshakeMessageBuilder;
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionHandshakePayloadTypes, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics24_host::identifier::{ClientId, ConnectionId};

use crate::types::message::SolomachineMessage;

pub struct BuildCosmosToSolomachineConnectionHandshakeMessage;

#[async_trait]
impl<Chain, Counterparty> ConnectionHandshakeMessageBuilder<Chain, Counterparty>
    for BuildCosmosToSolomachineConnectionHandshakeMessage
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = SolomachineMessage,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasErrorType,
    Counterparty: HasConnectionHandshakePayloadTypes<
            Chain,
            ConnectionOpenInitPayload = CosmosConnectionOpenInitPayload,
            ConnectionOpenTryPayload = CosmosConnectionOpenTryPayload,
            ConnectionOpenAckPayload = CosmosConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = CosmosConnectionOpenConfirmPayload,
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

    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &ConnectionId,
        _counterparty_connection_id: &ConnectionId,
        counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message = SolomachineMessage::CosmosConnectionOpenAck(Box::new(counterparty_payload));

        Ok(message)
    }

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
