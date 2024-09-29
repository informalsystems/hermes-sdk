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
