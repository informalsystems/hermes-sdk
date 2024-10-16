use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

#[derive_component(PacketSenderComponent, PacketSender<Chain>)]
#[async_trait]
pub trait CanSendPacket<Counterparty>:
    HasPacketHeaderType<Counterparty>
    + HasPayloadType<Counterparty>
    + HasPacketType<Counterparty>
    + HasErrorType
{
    async fn send_packet(
        &self,
        packet_header: &Self::PacketHeader,
        payloads: Vec<Self::Payload>,
    ) -> Result<Self::Packet, Self::Error>;
}
