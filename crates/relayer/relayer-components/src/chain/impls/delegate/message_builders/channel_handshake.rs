use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::message_builders::channel_handshake::{
    ChannelOpenAckMessageBuilder, ChannelOpenConfirmMessageBuilder, ChannelOpenInitMessageBuilder,
    ChannelOpenTryMessageBuilder,
};
use crate::chain::traits::types::channel::{
    HasChannelOpenAckPayloadType, HasChannelOpenConfirmPayloadType, HasChannelOpenTryPayloadType,
    HasInitChannelOptionsType,
};
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct DelegateBuildChannelHandshakeMessage<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ChannelOpenInitMessageBuilder<Chain, Counterparty>
    for DelegateBuildChannelHandshakeMessage<Components>
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
    for DelegateBuildChannelHandshakeMessage<Components>
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
    for DelegateBuildChannelHandshakeMessage<Components>
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
    ChannelOpenConfirmMessageBuilder<Chain, Counterparty>
    for DelegateBuildChannelHandshakeMessage<Components>
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
