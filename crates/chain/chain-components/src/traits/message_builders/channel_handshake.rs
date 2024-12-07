use cgp::core::component::UseDelegate;
use cgp::prelude::*;

use crate::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasInitChannelOptionsType,
};
use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  name: ChannelOpenInitMessageBuilderComponent,
  provider: ChannelOpenInitMessageBuilder,
  context: Chain,
}]
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

#[cgp_component {
  name: ChannelOpenTryMessageBuilderComponent,
  provider: ChannelOpenTryMessageBuilder,
  context: Chain,
}]
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

#[cgp_component {
  name: ChannelOpenAckMessageBuilderComponent,
  provider: ChannelOpenAckMessageBuilder,
  context: Chain,
}]
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

#[cgp_component {
  name: ChannelOpenConfirmMessageBuilderComponent,
  provider: ChannelOpenConfirmMessageBuilder,
  context: Chain,
}]
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

impl<Chain, Counterparty, Components, Delegate> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasInitChannelOptionsType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasIbcChainTypes<Chain>,
    Delegate: ChannelOpenInitMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_channel_open_init_message(
        chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &Chain::InitChannelOptions,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_channel_open_init_message(
            chain,
            port_id,
            counterparty_port_id,
            init_channel_options,
        )
        .await
    }
}

impl<Chain, Counterparty, Components, Delegate> ChannelOpenTryMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasChannelOpenTryPayloadType<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ChannelOpenTryMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_channel_open_try_message(
        chain: &Chain,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenTryPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_channel_open_try_message(
            chain,
            port_id,
            counterparty_port_id,
            counterparty_channel_id,
            counterparty_payload,
        )
        .await
    }
}

impl<Chain, Counterparty, Components, Delegate> ChannelOpenAckMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasChannelOpenAckPayloadType<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ChannelOpenAckMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_channel_open_ack_message(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_channel_open_ack_message(
            chain,
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_payload,
        )
        .await
    }
}

impl<Chain, Counterparty, Components, Delegate>
    ChannelOpenConfirmMessageBuilder<Chain, Counterparty> for UseDelegate<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasChannelOpenConfirmPayloadType<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ChannelOpenConfirmMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_channel_open_confirm_message(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_channel_open_confirm_message(
            chain,
            port_id,
            channel_id,
            counterparty_payload,
        )
        .await
    }
}
