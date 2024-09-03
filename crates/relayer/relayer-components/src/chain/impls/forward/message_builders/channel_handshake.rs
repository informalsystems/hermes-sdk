use cgp::core::inner::HasInner;
use cgp::prelude::{Async, CanRaiseError};

use crate::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenInitMessage, CanBuildChannelOpenTryMessage, ChannelOpenAckMessageBuilder,
    ChannelOpenConfirmMessageBuilder, ChannelOpenInitMessageBuilder, ChannelOpenTryMessageBuilder,
};
use crate::chain::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasInitChannelOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct ForwardChannelHandshakeBuilder;

impl<Chain, Counterparty, InChain, Message, PortId, InitChannelOptions>
    ChannelOpenInitMessageBuilder<Chain, Counterparty> for ForwardChannelHandshakeBuilder
where
    Chain: HasInitChannelOptionsType<Counterparty, InitChannelOptions = InitChannelOptions>
        + HasIbcChainTypes<Counterparty, Message = Message, PortId = PortId>
        + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty:
        HasIbcChainTypes<Chain, PortId = PortId> + HasIbcChainTypes<InChain, PortId = PortId>,
    InChain: CanBuildChannelOpenInitMessage<Counterparty, InitChannelOptions = InitChannelOptions>
        + HasIbcChainTypes<Counterparty, Message = Message, PortId = PortId>,
    PortId: Async,
    Message: Async,
    InitChannelOptions: Async,
{
    async fn build_channel_open_init_message(
        chain: &Chain,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        init_channel_options: &InitChannelOptions,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_channel_open_init_message(port_id, counterparty_port_id, init_channel_options)
            .await
            .map_err(Chain::raise_error)
    }
}

impl<Chain, Counterparty, InChain, Message, ChannelId, PortId, ChannelOpenTryPayload>
    ChannelOpenTryMessageBuilder<Chain, Counterparty> for ForwardChannelHandshakeBuilder
where
    Chain: HasIbcChainTypes<Counterparty, Message = Message, ChannelId = ChannelId, PortId = PortId>
        + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasChannelOpenTryPayloadType<Chain, ChannelOpenTryPayload = ChannelOpenTryPayload>
        + HasChannelOpenTryPayloadType<InChain, ChannelOpenTryPayload = ChannelOpenTryPayload>
        + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasIbcChainTypes<InChain, ChannelId = ChannelId, PortId = PortId>,
    InChain: CanBuildChannelOpenTryMessage<Counterparty>
        + HasIbcChainTypes<Counterparty, Message = Message, ChannelId = ChannelId, PortId = PortId>,
    ChannelId: Async,
    PortId: Async,
    ChannelOpenTryPayload: Async,
    Message: Async,
{
    async fn build_channel_open_try_message(
        chain: &Chain,
        port_id: &PortId,
        counterparty_port_id: &PortId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: ChannelOpenTryPayload,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_channel_open_try_message(
                port_id,
                counterparty_port_id,
                counterparty_channel_id,
                counterparty_payload,
            )
            .await
            .map_err(Chain::raise_error)
    }
}

impl<Chain, Counterparty, InChain, Message, ChannelId, PortId, ChannelOpenAckPayload>
    ChannelOpenAckMessageBuilder<Chain, Counterparty> for ForwardChannelHandshakeBuilder
where
    Chain: HasIbcChainTypes<Counterparty, Message = Message, ChannelId = ChannelId, PortId = PortId>
        + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasChannelOpenAckPayloadType<Chain, ChannelOpenAckPayload = ChannelOpenAckPayload>
        + HasChannelOpenAckPayloadType<InChain, ChannelOpenAckPayload = ChannelOpenAckPayload>
        + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasIbcChainTypes<InChain, ChannelId = ChannelId, PortId = PortId>,
    InChain: CanBuildChannelOpenAckMessage<Counterparty>
        + HasIbcChainTypes<Counterparty, Message = Message, ChannelId = ChannelId, PortId = PortId>,
    ChannelId: Async,
    PortId: Async,
    ChannelOpenAckPayload: Async,
    Message: Async,
{
    async fn build_channel_open_ack_message(
        chain: &Chain,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_channel_id: &ChannelId,
        counterparty_payload: ChannelOpenAckPayload,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_channel_open_ack_message(
                port_id,
                channel_id,
                counterparty_channel_id,
                counterparty_payload,
            )
            .await
            .map_err(Chain::raise_error)
    }
}

impl<Chain, Counterparty, InChain, Message, ChannelId, PortId, ChannelOpenConfirmPayload>
    ChannelOpenConfirmMessageBuilder<Chain, Counterparty> for ForwardChannelHandshakeBuilder
where
    Chain: HasIbcChainTypes<Counterparty, Message = Message, ChannelId = ChannelId, PortId = PortId>
        + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasChannelOpenConfirmPayloadType<
            Chain,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload,
        > + HasChannelOpenConfirmPayloadType<
            InChain,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload,
        > + HasIbcChainTypes<Chain, ChannelId = ChannelId, PortId = PortId>
        + HasIbcChainTypes<InChain, ChannelId = ChannelId, PortId = PortId>,
    InChain: CanBuildChannelOpenConfirmMessage<Counterparty>
        + HasIbcChainTypes<Counterparty, Message = Message, ChannelId = ChannelId, PortId = PortId>,
    ChannelId: Async,
    PortId: Async,
    ChannelOpenConfirmPayload: Async,
    Message: Async,
{
    async fn build_channel_open_confirm_message(
        chain: &Chain,
        port_id: &PortId,
        channel_id: &ChannelId,
        counterparty_payload: ChannelOpenConfirmPayload,
    ) -> Result<Message, Chain::Error> {
        chain
            .inner()
            .build_channel_open_confirm_message(port_id, channel_id, counterparty_payload)
            .await
            .map_err(Chain::raise_error)
    }
}
