use cgp::prelude::*;

use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};
use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  provider: ConnectionOpenInitPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildConnectionOpenInitPayload<Counterparty>:
    HasConnectionOpenInitPayloadType<Counterparty> + HasClientStateType<Counterparty> + HasErrorType
{
    async fn build_connection_open_init_payload(
        &self,
        client_state: &Self::ClientState,
    ) -> Result<Self::ConnectionOpenInitPayload, Self::Error>;
}

#[cgp_component {
  provider: ConnectionOpenTryPayloadBuilder,
  context: Chain,
}]
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

#[cgp_component {
  provider: ConnectionOpenAckPayloadBuilder,
  context: Chain,
}]
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

#[cgp_component {
  provider: ConnectionOpenConfirmPayloadBuilder,
  context: Chain,
}]
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
