use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilder;
use ibc_relayer_components::chain::traits::components::receive_packet_payload_builder::ReceivePacketPayloadBuilder;
use ibc_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayload;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

#[async_trait]
impl<Chain, Counterparty> HasReceivePacketPayload<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type ReceivePacketPayload = Chain::ReceivePacketPayload;
}

#[async_trait]
impl<Chain, Counterparty>
    ReceivePacketPayloadBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_receive_packet_payload(
        chain: &OfaChainWrapper<Chain>,
        client_state: &Chain::ClientState,
        height: &Chain::Height,
        packet: &Chain::OutgoingPacket,
    ) -> Result<Chain::ReceivePacketPayload, Chain::Error> {
        chain
            .chain
            .build_receive_packet_payload(client_state, height, packet)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ReceivePacketMessageBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_receive_packet_message(
        chain: &OfaChainWrapper<Chain>,
        packet: &Chain::IncomingPacket,
        payload: Counterparty::ReceivePacketPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_receive_packet_message(packet, payload)
            .await
    }
}
