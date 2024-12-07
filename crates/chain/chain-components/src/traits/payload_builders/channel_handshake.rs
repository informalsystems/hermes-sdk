use cgp::prelude::*;

use crate::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
};
use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  name: ChannelOpenTryPayloadBuilderComponent,
  provider: ChannelOpenTryPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenTryPayload<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasChannelOpenTryPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasErrorType
{
    async fn build_channel_open_try_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenTryPayload, Self::Error>;
}

#[cgp_component {
  name: ChannelOpenAckPayloadBuilderComponent,
  provider: ChannelOpenAckPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenAckPayload<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasChannelOpenAckPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasErrorType
{
    async fn build_channel_open_ack_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenAckPayload, Self::Error>;
}

#[cgp_component {
  name: ChannelOpenConfirmPayloadBuilderComponent,
  provider: ChannelOpenConfirmPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenConfirmPayload<Counterparty>:
    HasIbcChainTypes<Counterparty>
    + HasChannelOpenConfirmPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasErrorType
{
    async fn build_channel_open_confirm_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenConfirmPayload, Self::Error>;
}
