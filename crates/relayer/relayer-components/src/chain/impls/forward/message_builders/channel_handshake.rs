use cgp_core::{Async, CanRaiseError, HasInner};

use crate::chain::traits::message_builders::channel_handshake::CanBuildChannelHandshakeMessages;
use crate::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilder;
use crate::chain::traits::types::channel::HasChannelHandshakePayloadTypes;
use crate::chain::traits::types::channel::HasInitChannelOptionsType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct ForwardChannelHandshakeBuilder;

impl<
        Chain,
        Counterparty,
        InChain,
        Message,
        ClientId,
        ConnectionId,
        ChannelId,
        PortId,
        InitChannelOptions,
        ChannelOpenTryPayload,
        ChannelOpenAckPayload,
        ChannelOpenConfirmPayload,
    > ChannelHandshakeMessageBuilder<Chain, Counterparty> for ForwardChannelHandshakeBuilder
where
    Chain: HasInitChannelOptionsType<Counterparty, InitChannelOptions = InitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasInner<Inner = InChain>
        + CanRaiseError<InChain::Error>,
    Counterparty: HasChannelHandshakePayloadTypes<
            Chain,
            ChannelOpenTryPayload = ChannelOpenTryPayload,
            ChannelOpenAckPayload = ChannelOpenAckPayload,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload,
        > + HasChannelHandshakePayloadTypes<
            InChain,
            ChannelOpenTryPayload = ChannelOpenTryPayload,
            ChannelOpenAckPayload = ChannelOpenAckPayload,
            ChannelOpenConfirmPayload = ChannelOpenConfirmPayload,
        > + HasIbcChainTypes<
            Chain,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            ChannelId = ChannelId,
            PortId = PortId,
        > + HasIbcChainTypes<
            InChain,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            ChannelId = ChannelId,
            PortId = PortId,
        >,
    InChain: CanBuildChannelHandshakeMessages<Counterparty, InitChannelOptions = InitChannelOptions>
        + HasIbcChainTypes<
            Counterparty,
            Message = Message,
            ClientId = ClientId,
            ConnectionId = ConnectionId,
            ChannelId = ChannelId,
            PortId = PortId,
        >,
    ClientId: Async,
    ConnectionId: Async,
    ChannelId: Async,
    PortId: Async,
    ChannelOpenTryPayload: Async,
    ChannelOpenAckPayload: Async,
    ChannelOpenConfirmPayload: Async,
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
