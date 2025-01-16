use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::counterparty::CanUseCounterparty;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;
use hermes_chain_type_components::traits::types::message::HasMessageType;

use crate::traits::types::channel::{
    ChannelOpenAckPayloadOf, ChannelOpenConfirmPayloadOf, ChannelOpenTryPayloadOf,
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasInitChannelOptionsType,
};
use crate::types::aliases::{ChannelIdOf, PortIdOf};

#[cgp_component {
  provider: ChannelOpenInitMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenInitMessage<Counterparty>:
    Sized
    + HasMessageType
    + HasPortIdType<Counterparty>
    + HasInitChannelOptionsType<Counterparty>
    + HasAsyncErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasPortIdType<Self>>
{
    async fn build_channel_open_init_message(
        &self,
        port_id: &Self::PortId,
        counterparty_port_id: &PortIdOf<Counterparty, Self>,
        init_channel_options: &Self::InitChannelOptions,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_component {
  provider: ChannelOpenTryMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenTryMessage<Counterparty>:
    HasMessageType
    + HasPortIdType<Counterparty>
    + HasAsyncErrorType
    + CanUseCounterparty<
        Counterparty,
        Counterparty: HasChannelIdType<Self>
                          + HasPortIdType<Self>
                          + HasChannelOpenTryPayloadType<Self>,
    >
{
    async fn build_channel_open_try_message(
        &self,
        port_id: &Self::PortId,
        counterparty_port_id: &PortIdOf<Counterparty, Self>,
        counterparty_channel_id: &ChannelIdOf<Counterparty, Self>,
        counterparty_payload: ChannelOpenTryPayloadOf<Counterparty, Self>,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_component {
  provider: ChannelOpenAckMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenAckMessage<Counterparty>:
    HasMessageType
    + HasPortIdType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasAsyncErrorType
    + CanUseCounterparty<
        Counterparty,
        Counterparty: HasChannelIdType<Self>
                          + HasPortIdType<Self>
                          + HasChannelOpenAckPayloadType<Self>,
    >
{
    async fn build_channel_open_ack_message(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        counterparty_channel_id: &ChannelIdOf<Counterparty, Self>,
        counterparty_payload: ChannelOpenAckPayloadOf<Counterparty, Self>,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_component {
  provider: ChannelOpenConfirmMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildChannelOpenConfirmMessage<Counterparty>:
    HasMessageType
    + HasPortIdType<Counterparty>
    + HasChannelIdType<Counterparty>
    + HasAsyncErrorType
    + CanUseCounterparty<Counterparty, Counterparty: HasChannelOpenConfirmPayloadType<Self>>
{
    async fn build_channel_open_confirm_message(
        &self,
        port_id: &Self::PortId,
        channel_id: &Self::ChannelId,
        counterparty_payload: ChannelOpenConfirmPayloadOf<Counterparty, Self>,
    ) -> Result<Self::Message, Self::Error>;
}

impl<Chain, Counterparty, Components, Delegate> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasMessageType
        + HasPortIdType<Counterparty>
        + HasInitChannelOptionsType<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasPortIdType<Chain>,
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
    Chain: HasMessageType + HasPortIdType<Counterparty> + HasAsyncErrorType,
    Counterparty:
        HasChannelIdType<Chain> + HasPortIdType<Chain> + HasChannelOpenTryPayloadType<Chain>,
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
    Chain: HasMessageType
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasAsyncErrorType,
    Counterparty:
        HasChannelIdType<Chain> + HasPortIdType<Chain> + HasChannelOpenAckPayloadType<Chain>,
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
    Chain: HasMessageType
        + HasPortIdType<Counterparty>
        + HasChannelIdType<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasChannelOpenConfirmPayloadType<Chain>,
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
