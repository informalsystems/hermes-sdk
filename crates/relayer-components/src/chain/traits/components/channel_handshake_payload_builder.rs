use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::chain::traits::types::channel::HasChannelHandshakePayloads;
use crate::chain::traits::types::client_state::HasClientStateType;
use crate::std_prelude::*;

#[derive_component(ChannelHandshakePayloadBuilderComponent, ChannelHandshakePayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildChannelHandshakePayloads<Counterparty>:
    HasChannelHandshakePayloads<Counterparty> + HasClientStateType<Counterparty> + HasErrorType
{
    async fn build_channel_open_try_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenTryPayload, Self::Error>;

    async fn build_channel_open_ack_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenAckPayload, Self::Error>;

    async fn build_channel_open_confirm_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenConfirmPayload, Self::Error>;
}
