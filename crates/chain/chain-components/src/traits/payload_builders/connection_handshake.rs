use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::connection_id::HasConnectionIdType;

use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::connection::{
    HasConnectionOpenAckPayloadType, HasConnectionOpenConfirmPayloadType,
    HasConnectionOpenInitPayloadType, HasConnectionOpenTryPayloadType,
};

#[cgp_component {
  provider: ConnectionOpenInitPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildConnectionOpenInitPayload<Counterparty>:
    HasConnectionOpenInitPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasAsyncErrorType
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
    HasHeightType
    + HasClientIdType<Counterparty>
    + HasConnectionIdType<Counterparty>
    + HasConnectionOpenTryPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasAsyncErrorType
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
    HasHeightType
    + HasClientIdType<Counterparty>
    + HasConnectionIdType<Counterparty>
    + HasConnectionOpenAckPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasAsyncErrorType
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
    HasHeightType
    + HasClientIdType<Counterparty>
    + HasConnectionIdType<Counterparty>
    + HasConnectionOpenConfirmPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasAsyncErrorType
{
    async fn build_connection_open_confirm_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenConfirmPayload, Self::Error>;
}
