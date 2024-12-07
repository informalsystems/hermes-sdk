use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

#[cgp_component {
  name: PacketSenderComponent,
  provider: PacketSender,
  context: Chain,
}]
#[async_trait]
pub trait CanSendPacket<Counterparty>:
    HasPacketHeaderType<Counterparty>
    + HasPayloadType<Counterparty>
    + HasPacketType<Counterparty>
    + HasErrorType
{
    async fn send_packet(
        &mut self,
        packet_header: &Self::PacketHeader,
        payloads: Vec<Self::Payload>,
    ) -> Result<Self::Packet, Self::Error>;
}
