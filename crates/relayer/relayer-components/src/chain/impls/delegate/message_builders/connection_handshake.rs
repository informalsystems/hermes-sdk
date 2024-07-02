use core::marker::PhantomData;

use cgp_core::prelude::*;

use crate::chain::traits::message_builders::connection_handshake::{
    ConnectionOpenAckMessageBuilder, ConnectionOpenConfirmMessageBuilder,
    ConnectionOpenInitMessageBuilder, ConnectionOpenTryMessageBuilder,
};
use crate::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct DelegateBuildConnectionHandshakeMessage<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate>
    ConnectionOpenInitMessageBuilder<Chain, Counterparty>
    for DelegateBuildConnectionHandshakeMessage<Components>
where
    Chain:
        HasInitConnectionOptionsType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasConnectionOpenInitPayloadType<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ConnectionOpenInitMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_connection_open_init_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Chain::InitConnectionOptions,
        counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_connection_open_init_message(
            chain,
            client_id,
            counterparty_client_id,
            init_connection_options,
            counterparty_payload,
        )
        .await
    }
}

impl<Chain, Counterparty, Components, Delegate> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for DelegateBuildConnectionHandshakeMessage<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasConnectionOpenTryPayloadType<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ConnectionOpenTryMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_connection_open_try_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_connection_open_try_message(
            chain,
            client_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_payload,
        )
        .await
    }
}

impl<Chain, Counterparty, Components, Delegate> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for DelegateBuildConnectionHandshakeMessage<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasConnectionOpenAckPayloadType<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ConnectionOpenAckMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_connection_open_ack_message(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_connection_open_ack_message(
            chain,
            connection_id,
            counterparty_connection_id,
            counterparty_payload,
        )
        .await
    }
}

impl<Chain, Counterparty, Components, Delegate>
    ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>
    for DelegateBuildConnectionHandshakeMessage<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasConnectionOpenConfirmPayloadType<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ConnectionOpenConfirmMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_connection_open_confirm_message(
        chain: &Chain,
        connection_id: &Chain::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_connection_open_confirm_message(chain, connection_id, counterparty_payload)
            .await
    }
}
