use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::channel_handshake_message_builder::ChannelHandshakeMessageBuilder;
use ibc_relayer_components::chain::traits::components::channel_handshake_payload_builder::ChannelHandshakePayloadBuilder;
use ibc_relayer_components::chain::traits::types::channel::{
    HasChannelHandshakePayloads, HasInitChannelOptionsType,
};
use ibc_relayer_components::chain::traits::types::ibc_events::channel::{
    HasChannelOpenInitEvent, HasChannelOpenTryEvent,
};

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

impl<Chain, Counterparty> HasChannelHandshakePayloads<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type ChannelOpenTryPayload = Chain::ChannelOpenTryPayload;

    type ChannelOpenAckPayload = Chain::ChannelOpenAckPayload;

    type ChannelOpenConfirmPayload = Chain::ChannelOpenConfirmPayload;
}

impl<Chain, Counterparty> HasInitChannelOptionsType<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type InitChannelOptions = Chain::InitChannelOptions;
}

impl<Chain, Counterparty> HasChannelOpenInitEvent<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    type ChannelOpenInitEvent = Chain::ChannelOpenInitEvent;

    fn try_extract_channel_open_init_event(
        event: Chain::Event,
    ) -> Option<Chain::ChannelOpenInitEvent> {
        Chain::try_extract_channel_open_init_event(event)
    }

    fn channel_open_init_event_channel_id(
        event: &Chain::ChannelOpenInitEvent,
    ) -> &Chain::ChannelId {
        Chain::channel_open_init_event_channel_id(event)
    }
}

impl<Chain, Counterparty> HasChannelOpenTryEvent<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    type ChannelOpenTryEvent = Chain::ChannelOpenTryEvent;

    fn try_extract_channel_open_try_event(
        event: Chain::Event,
    ) -> Option<Chain::ChannelOpenTryEvent> {
        Chain::try_extract_channel_open_try_event(event)
    }

    fn channel_open_try_event_channel_id(event: &Chain::ChannelOpenTryEvent) -> &Chain::ChannelId {
        Chain::channel_open_try_event_channel_id(event)
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ChannelHandshakePayloadBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_channel_open_try_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
    ) -> Result<Chain::ChannelOpenTryPayload, Chain::Error> {
        chain
            .chain
            .build_channel_open_try_payload(client_state, height, port_id, channel_id)
            .await
    }

    async fn build_channel_open_ack_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
    ) -> Result<Chain::ChannelOpenAckPayload, Chain::Error> {
        chain
            .chain
            .build_channel_open_ack_payload(client_state, height, port_id, channel_id)
            .await
    }

    async fn build_channel_open_confirm_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
    ) -> Result<Chain::ChannelOpenConfirmPayload, Chain::Error> {
        chain
            .chain
            .build_channel_open_confirm_payload(client_state, height, port_id, channel_id)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ChannelHandshakeMessageBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_channel_open_init_message(
        chain: &OfaChainWrapper<Chain>,
        port_id: &Chain::PortId,
        counterparty_port_id: &Counterparty::PortId,
        init_channel_options: &Chain::InitChannelOptions,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_channel_open_init_message(port_id, counterparty_port_id, init_channel_options)
            .await
    }

    async fn build_channel_open_try_message(
        chain: &OfaChainWrapper<Chain>,
        dst_port_id: &Chain::PortId,
        src_port_id: &Counterparty::PortId,
        src_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenTryPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_channel_open_try_message(
                dst_port_id,
                src_port_id,
                src_channel_id,
                counterparty_payload,
            )
            .await
    }

    async fn build_channel_open_ack_message(
        chain: &OfaChainWrapper<Chain>,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_channel_id: &Counterparty::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenAckPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_channel_open_ack_message(
                port_id,
                channel_id,
                counterparty_channel_id,
                counterparty_payload,
            )
            .await
    }

    async fn build_channel_open_confirm_message(
        chain: &OfaChainWrapper<Chain>,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        counterparty_payload: Counterparty::ChannelOpenConfirmPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_channel_open_confirm_message(port_id, channel_id, counterparty_payload)
            .await
    }
}
