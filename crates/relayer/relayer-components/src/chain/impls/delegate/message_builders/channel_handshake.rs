use core::marker::PhantomData;

use cgp_core::prelude::HasErrorType;
use cgp_core::DelegateComponent;

use crate::chain::traits::message_builders::channel_handshake::ChannelHandshakeMessageBuilder;
use crate::chain::traits::types::channel::HasChannelHandshakePayloadTypes;
use crate::chain::traits::types::channel::HasInitChannelOptionsType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub struct DelegateBuildChannelHandshakeMessage<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> ChannelHandshakeMessageBuilder<Chain, Counterparty>
    for DelegateBuildChannelHandshakeMessage<Components>
where
    Chain: HasInitChannelOptionsType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType,
    Counterparty: HasChannelHandshakePayloadTypes<Chain> + HasIbcChainTypes<Chain>,
    Delegate: ChannelHandshakeMessageBuilder<Chain, Counterparty>,
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
