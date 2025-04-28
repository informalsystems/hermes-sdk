use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasConnectionIdType, HasMessageType};
use hermes_prelude::*;

use crate::traits::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
    HasInitConnectionOptionsType,
};

#[cgp_component {
  provider: ConnectionOpenInitMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildConnectionOpenInitMessage<Counterparty>:
    HasInitConnectionOptionsType<Counterparty>
    + HasClientIdType<Counterparty>
    + HasMessageType
    + HasAsyncErrorType
where
    Counterparty: HasConnectionOpenInitPayloadType<Self> + HasClientIdType<Self>,
{
    async fn build_connection_open_init_message(
        &self,
        client_id: &Self::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        init_connection_options: &Self::InitConnectionOptions,
        counterparty_payload: Counterparty::ConnectionOpenInitPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_component {
  provider: ConnectionOpenTryMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildConnectionOpenTryMessage<Counterparty>:
    HasMessageType + HasClientIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty:
        HasConnectionOpenTryPayloadType<Self> + HasClientIdType<Self> + HasConnectionIdType<Self>,
{
    async fn build_connection_open_try_message(
        &self,
        client_id: &Self::ClientId,
        counterparty_client_id: &Counterparty::ClientId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenTryPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_component {
  provider: ConnectionOpenAckMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildConnectionOpenAckMessage<Counterparty>:
    HasMessageType + HasConnectionIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasConnectionOpenAckPayloadType<Self> + HasConnectionIdType<Self>,
{
    async fn build_connection_open_ack_message(
        &self,
        connection_id: &Self::ConnectionId,
        counterparty_connection_id: &Counterparty::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenAckPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_component {
  provider: ConnectionOpenConfirmMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildConnectionOpenConfirmMessage<Counterparty>:
    HasMessageType + HasConnectionIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasConnectionOpenConfirmPayloadType<Self>,
{
    async fn build_connection_open_confirm_message(
        &self,
        connection_id: &Self::ConnectionId,
        counterparty_payload: Counterparty::ConnectionOpenConfirmPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_provider(ConnectionOpenInitMessageBuilderComponent)]
impl<Chain, Counterparty, Components, Delegate>
    ConnectionOpenInitMessageBuilder<Chain, Counterparty> for UseDelegate<Components>
where
    Chain: HasInitConnectionOptionsType<Counterparty>
        + HasClientIdType<Counterparty>
        + HasMessageType
        + HasAsyncErrorType,
    Counterparty: HasConnectionOpenInitPayloadType<Chain> + HasClientIdType<Chain>,
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

#[cgp_provider(ConnectionOpenTryMessageBuilderComponent)]
impl<Chain, Counterparty, Components, Delegate> ConnectionOpenTryMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasMessageType + HasClientIdType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasConnectionOpenTryPayloadType<Chain>
        + HasClientIdType<Chain>
        + HasConnectionIdType<Chain>,
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

#[cgp_provider(ConnectionOpenAckMessageBuilderComponent)]
impl<Chain, Counterparty, Components, Delegate> ConnectionOpenAckMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasMessageType + HasConnectionIdType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasConnectionOpenAckPayloadType<Chain> + HasConnectionIdType<Chain>,
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

#[cgp_provider(ConnectionOpenConfirmMessageBuilderComponent)]
impl<Chain, Counterparty, Components, Delegate>
    ConnectionOpenConfirmMessageBuilder<Chain, Counterparty> for UseDelegate<Components>
where
    Chain: HasMessageType + HasConnectionIdType<Counterparty> + HasAsyncErrorType,
    Counterparty: HasConnectionOpenConfirmPayloadType<Chain>,
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
