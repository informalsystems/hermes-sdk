use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilder;
use ibc_relayer_components::chain::traits::components::ack_packet_payload_builder::AckPacketPayloadBuilder;
use ibc_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty> HasAckPacketPayload<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type AckPacketPayload = Chain::AckPacketPayload;
}

#[async_trait]
impl<Chain, Counterparty>
    AckPacketPayloadBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>> for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_ack_packet_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Chain::IncomingPacket,
        ack: &Chain::WriteAckEvent,
    ) -> Result<Chain::AckPacketPayload, Chain::Error> {
        chain
            .chain
            .build_ack_packet_payload(client_state, height, packet, ack)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    AckPacketMessageBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>> for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_ack_packet_message(
        chain: &OfaChainWrapper<Chain>,
        packet: &Chain::OutgoingPacket,
        payload: Counterparty::AckPacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain.chain.build_ack_packet_message(packet, payload).await
    }
}
