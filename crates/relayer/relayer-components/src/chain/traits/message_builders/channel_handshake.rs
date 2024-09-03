use cgp::prelude::*;

use crate::chain::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasInitChannelOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ChannelOpenInitMessageBuilderComponent, ChannelOpenInitMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildChannelOpenInitMessage<Counterparty>:
    HasInitChannelOptionsType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasIbcChainTypes<Self>,
{
    async fn build_channel_open_init_message(
        &self,
        port_id: &Self::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &Self::InitChannelOptions,
    ) -> Result<Self::Message, Self::Error>;
}

#[derive_component(ChannelOpenTryMessageBuilderComponent, ChannelOpenTryMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildChannelOpenTryMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasChannelOpenTryPayloadType<Self> + HasIbcChainTypes<Self>,
{
    async fn build_channel_open_try_message(
        &self,
        port_id: &Self::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenTryPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[derive_component(ChannelOpenAckMessageBuilderComponent, ChannelOpenAckMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildChannelOpenAckMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasChannelOpenAckPayloadType<Self> + HasIbcChainTypes<Self>,
{
    async fn build_channel_open_ack_message(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenAckPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[derive_component(ChannelOpenConfirmMessageBuilderComponent, ChannelOpenConfirmMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildChannelOpenConfirmMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasChannelOpenConfirmPayloadType<Self> + HasIbcChainTypes<Self>,
{
    async fn build_channel_open_confirm_message(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenConfirmPayload,
    ) -> Result<Self::Message, Self::Error>;
}
