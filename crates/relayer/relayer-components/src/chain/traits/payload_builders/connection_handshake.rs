use cgp::prelude::*;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ConnectionOpenInitPayloadBuilderComponent, ConnectionOpenInitPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenInitPayload<Counterparty>:
    HasConnectionOpenInitPayloadType<Counterparty> + HasClientStateType<Counterparty> + HasErrorType
{
    async fn build_connection_open_init_payload(
        &self,
        client_state: &Self::ClientState,
    ) -> Result<Self::ConnectionOpenInitPayload, Self::Error>;
}

#[derive_component(ConnectionOpenTryPayloadBuilderComponent, ConnectionOpenTryPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenTryPayload<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasConnectionOpenTryPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasErrorType
{
    async fn build_connection_open_try_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenTryPayload, Self::Error>;
}

#[derive_component(ConnectionOpenAckPayloadBuilderComponent, ConnectionOpenAckPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenAckPayload<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasConnectionOpenAckPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasErrorType
{
    async fn build_connection_open_ack_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenAckPayload, Self::Error>;
}

#[derive_component(ConnectionOpenConfirmPayloadBuilderComponent, ConnectionOpenConfirmPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionOpenConfirmPayload<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasConnectionOpenConfirmPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasErrorType
{
    async fn build_connection_open_confirm_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenConfirmPayload, Self::Error>;
}
