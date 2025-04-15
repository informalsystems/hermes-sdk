use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasChannelIdType, HasHeightType, HasPortIdType};

use crate::traits::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasClientStateType,
};

#[cgp_component {
  provider: ChannelOpenTryPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenTryPayload<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasChannelOpenTryPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasAsyncErrorType
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
  provider: ChannelOpenAckPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenAckPayload<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasChannelOpenAckPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasAsyncErrorType
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
  provider: ChannelOpenConfirmPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenConfirmPayload<Counterparty>:
    HasHeightType
    + HasChannelIdType<Counterparty>
    + HasPortIdType<Counterparty>
    + HasChannelOpenConfirmPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasAsyncErrorType
{
    async fn build_channel_open_confirm_payload(
        &self,
        client_state: &Self::ClientState,
        height: &Self::Height,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
    ) -> Result<Self::ChannelOpenConfirmPayload, Self::Error>;
}
