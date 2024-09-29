use cgp::core::component::DelegateTo;
use cgp::prelude::*;

use crate::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};
use crate::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ConnectionOpenInitMessageBuilderComponent, ConnectionOpenInitMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenInitMessage<Counterparty>:
    HasInitConnectionOptionsType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasConnectionOpenInitPayloadType<Self> + HasIbcChainTypes<Self>,
{
    async fn build_connection_open_init_message(
        &self,
        client_id: &Self::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Self::InitConnectionOptions,
        counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[derive_component(ConnectionOpenTryMessageBuilderComponent, ConnectionOpenTryMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenTryMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasConnectionOpenTryPayloadType<Self> + HasIbcChainTypes<Self>,
{
    async fn build_connection_open_try_message(
        &self,
        client_id: &Self::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[derive_component(ConnectionOpenAckMessageBuilderComponent, ConnectionOpenAckMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenAckMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasConnectionOpenAckPayloadType<Self> + HasIbcChainTypes<Self>,
{
    async fn build_connection_open_ack_message(
        &self,
        connection_id: &Self::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[derive_component(ConnectionOpenConfirmMessageBuilderComponent, ConnectionOpenConfirmMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenConfirmMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasConnectionOpenConfirmPayloadType<Self>,
{
    async fn build_connection_open_confirm_message(
        &self,
        connection_id: &Self::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<Self::Message, Self::Error>;
}

impl<Chain, Counterparty, Components, Delegate>
    ConnectionOpenInitMessageBuilder<Chain, Counterparty> for DelegateTo<Components>
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
    for DelegateTo<Components>
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
    for DelegateTo<Components>
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
    ConnectionOpenConfirmMessageBuilder<Chain, Counterparty> for DelegateTo<Components>
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
