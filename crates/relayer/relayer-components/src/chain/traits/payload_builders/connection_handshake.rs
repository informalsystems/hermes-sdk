use cgp_core::prelude::*;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::connection::HasConnectionHandshakePayloadTypes;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ConnectionHandshakePayloadBuilderComponent, ConnectionHandshakePayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildConnectionHandshakePayloads<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasConnectionHandshakePayloadTypes<Counterparty>
    + HasClientStateType<Counterparty>
    + HasErrorType
{
    async fn build_connection_open_init_payload(
        &self,
        client_state: &Self::ClientState,
    ) -> Result<Self::ConnectionOpenInitPayload, Self::Error>;

    async fn build_connection_open_try_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenTryPayload, Self::Error>;

    async fn build_connection_open_ack_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenAckPayload, Self::Error>;

    async fn build_connection_open_confirm_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        client_id: &Self::ClientId,
        connection_id: &Self::ConnectionId,
    ) -> Result<Self::ConnectionOpenConfirmPayload, Self::Error>;
}
