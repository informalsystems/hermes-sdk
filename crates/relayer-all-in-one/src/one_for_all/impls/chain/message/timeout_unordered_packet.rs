use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    TimeoutUnorderedPacketMessageBuilder, TimeoutUnorderedPacketPayloadBuilder,
};
use ibc_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

impl<Chain, Counterparty> HasTimeoutUnorderedPacketPayload<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type TimeoutUnorderedPacketPayload = Chain::TimeoutUnorderedPacketPayload;
}

#[async_trait]
impl<Chain, Counterparty>
    TimeoutUnorderedPacketPayloadBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_timeout_unordered_packet_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Chain::IncomingPacket,
    ) -> Result<Chain::TimeoutUnorderedPacketPayload, Chain::Error> {
        chain
            .chain
            .build_timeout_unordered_packet_payload(client_state, height, packet)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    TimeoutUnorderedPacketMessageBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_timeout_unordered_packet_message(
        chain: &OfaChainWrapper<Chain>,
        packet: &Chain::OutgoingPacket,
        payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_timeout_unordered_packet_message(packet, payload)
            .await
    }
}
