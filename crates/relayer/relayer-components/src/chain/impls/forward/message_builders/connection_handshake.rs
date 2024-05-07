use cgp_core::{Async, CanRaiseError, HasInner};

use crate::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionHandshakeMessages, ConnectionHandshakeMessageBuilder,
};
use crate::chain::traits::types::connection::{
    HasConnectionHandshakePayloadTypes, HasInitConnectionOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct ForwardConnectionHandshakeBuilder;

impl<
        Chain,
        Counterparty,
        InChain,
        Message,
        ClientId,
        ConnectionId,
        InitConnectionOptions,
        ConnectionOpenInitPayload,
        ConnectionOpenTryPayload,
        ConnectionOpenAckPayload,
        ConnectionOpenConfirmPayload,
    > ConnectionHandshakeMessageBuilder<Chain, Counterparty> for ForwardConnectionHandshakeBuilder
where
    Chain: HasInitConnectionOptionsType<Counterparty, InitConnectionOptions = InitConnectionOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasConnectionHandshakePayloadTypes<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload,
        > + HasConnectionHandshakePayloadTypes<
            InChain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasIbcChainTypes<InChain, ClientId = ClientId, ConnectionId = ConnectionId>,
    InChain: CanBuildConnectionHandshakeMessages<
            Counterparty,
            InitConnectionOptions = InitConnectionOptions,
        > + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        >,
    ClientId: Async,
    ConnectionId: Async,
    ConnectionOpenInitPayload: Async,
    ConnectionOpenTryPayload: Async,
    ConnectionOpenAckPayload: Async,
    ConnectionOpenConfirmPayload: Async,
    Message: Async,
    InitConnectionOptions: Async,
{
    async fn build_connection_open_init_message(
        chain: &Chain,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        init_connection_options: &InitConnectionOptions,
        counterparty_payload: ConnectionOpenInitPayload,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_connection_open_init_message(
                client_id,
                counterparty_client_id,
                init_connection_options,
                counterparty_payload,
            )
            .await
            .map_err(Chain::raise_error)
    }

    async fn build_connection_open_try_message(
        chain: &Chain,
        client_id: &ClientId,
        counterparty_client_id: &ClientId,
        counterparty_connection_id: &ConnectionId,
        counterparty_payload: ConnectionOpenTryPayload,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_connection_open_try_message(
                client_id,
                counterparty_client_id,
                counterparty_connection_id,
                counterparty_payload,
            )
            .await
            .map_err(Chain::raise_error)
    }

    async fn build_connection_open_ack_message(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &ConnectionId,
        counterparty_payload: ConnectionOpenAckPayload,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_connection_open_ack_message(
                connection_id,
                counterparty_connection_id,
                counterparty_payload,
            )
            .await
            .map_err(Chain::raise_error)
    }

    async fn build_connection_open_confirm_message(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: ConnectionOpenConfirmPayload,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_connection_open_confirm_message(connection_id, counterparty_payload)
            .await
            .map_err(Chain::raise_error)
    }
}
