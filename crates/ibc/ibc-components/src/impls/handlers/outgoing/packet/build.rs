use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::builders::packet::CanBuildPacket;
use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::handlers::outgoing::packet::PacketSender;
use crate::traits::nonce::CanAllocatePacketNonce;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

pub struct AllocateNonceAndBuildPacket;

#[async_trait]
impl<Chain, Counterparty> PacketSender<Chain, Counterparty> for AllocateNonceAndBuildPacket
where
    Chain: HasPacketChannelIds<Counterparty>
        + HasPayloadType<Counterparty>
        + HasPacketType<Counterparty>
        + CanBuildPacket<Counterparty>
        + CanAllocatePacketNonce<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
{
    async fn send_packet(
        chain: &mut Chain,
        packet_header: &Chain::PacketHeader,
        payloads: Vec<Chain::Payload>,
    ) -> Result<Chain::Packet, Chain::Error> {
        let src_channel_id = Chain::packet_src_channel_id(packet_header);
        let dst_channel_id = Chain::packet_dst_channel_id(packet_header);

        let nonce = chain
            .allocate_packet_nonce(src_channel_id, dst_channel_id)
            .await?;

        let packet = Chain::build_packet(packet_header, nonce, payloads)?;

        Ok(packet)
    }
}
