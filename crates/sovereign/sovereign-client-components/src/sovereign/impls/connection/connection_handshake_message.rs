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

use crate::sovereign::types::message::SovereignMessage;

pub struct BuildCosmosConnectionHandshakeMessageOnSovereign;

impl<Chain, Counterparty> ConnectionHandshakeMessageBuilder<Chain, Counterparty>
    for BuildCosmosConnectionHandshakeMessageOnSovereign
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = SovereignMessage,
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
        _client_id: &Chain::ClientId,
        _counterparty_client_id: &Counterparty::ClientId,
        _init_connection_options: &Chain::InitConnectionOptions,
        _counterparty_payload: CosmosConnectionOpenInitPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_try_message(
        _chain: &Chain,
        _client_id: &Chain::ClientId,
        _counterparty_client_id: &Counterparty::ClientId,
        _counterparty_connection_id: &Counterparty::ConnectionId,
        _counterparty_payload: CosmosConnectionOpenTryPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_ack_message(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _counterparty_connection_id: &Counterparty::ConnectionId,
        _counterparty_payload: CosmosConnectionOpenAckPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }

    async fn build_connection_open_confirm_message(
        _chain: &Chain,
        _connection_id: &Chain::ConnectionId,
        _counterparty_payload: CosmosConnectionOpenConfirmPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }
}
