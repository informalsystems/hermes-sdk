use cgp_core::inner::HasInner;
use cgp_core::prelude::{Async, CanRaiseError};

use crate::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use crate::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
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
    > ConnectionOpenInitMessageBuilder<Chain, Counterparty> for ForwardConnectionHandshakeBuilder
where
    Chain: HasInitConnectionOptionsType<Counterparty, InitConnectionOptions = InitConnectionOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasConnectionOpenInitPayloadType<
            Chain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload,
        > + HasConnectionOpenInitPayloadType<
            InChain,
            ConnectionOpenInitPayload = ConnectionOpenInitPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasIbcChainTypes<InChain, ClientId = ClientId, ConnectionId = ConnectionId>,
    InChain: CanBuildConnectionOpenInitMessage<
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
}

impl<Chain, Counterparty, InChain, Message, ClientId, ConnectionId, ConnectionOpenTryPayload>
    ConnectionOpenTryMessageBuilder<Chain, Counterparty> for ForwardConnectionHandshakeBuilder
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasConnectionOpenTryPayloadType<Chain, ConnectionOpenTryPayload = ConnectionOpenTryPayload>
        + HasConnectionOpenTryPayloadType<
            InChain,
            ConnectionOpenTryPayload = ConnectionOpenTryPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasIbcChainTypes<InChain, ClientId = ClientId, ConnectionId = ConnectionId>,
    InChain: CanBuildConnectionOpenTryMessage<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        >,
    ClientId: Async,
    ConnectionId: Async,
    ConnectionOpenTryPayload: Async,
    Message: Async,
{
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
}

impl<Chain, Counterparty, InChain, Message, ClientId, ConnectionId, ConnectionOpenAckPayload>
    ConnectionOpenAckMessageBuilder<Chain, Counterparty> for ForwardConnectionHandshakeBuilder
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasConnectionOpenAckPayloadType<Chain, ConnectionOpenAckPayload = ConnectionOpenAckPayload>
        + HasConnectionOpenAckPayloadType<
            InChain,
            ConnectionOpenAckPayload = ConnectionOpenAckPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasIbcChainTypes<InChain, ClientId = ClientId, ConnectionId = ConnectionId>,
    InChain: CanBuildConnectionOpenAckMessage<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        >,
    ClientId: Async,
    ConnectionId: Async,
    ConnectionOpenAckPayload: Async,
    Message: Async,
{
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
}

impl<
        Chain,
        Counterparty,
        InChain,
        Message,
        ClientId,
        ConnectionId,
        ConnectionOpenConfirmPayload,
    > ConnectionOpenConfirmMessageBuilder<Chain, Counterparty> for ForwardConnectionHandshakeBuilder
where
    Chain: HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        > + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasConnectionOpenConfirmPayloadType<
            Chain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload,
        > + HasConnectionOpenConfirmPayloadType<
            InChain,
            ConnectionOpenConfirmPayload = ConnectionOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ClientId = ClientId, ConnectionId = ConnectionId>
        + HasIbcChainTypes<InChain, ClientId = ClientId, ConnectionId = ConnectionId>,
    InChain: CanBuildConnectionOpenConfirmMessage<Counterparty>
        + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
        >,
    ClientId: Async,
    ConnectionId: Async,
    ConnectionOpenConfirmPayload: Async,
    Message: Async,
{
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
