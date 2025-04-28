use alloc::vec::Vec;

use hermes_prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

#[cgp_component {
  provider: PacketSender,
  context: Chain,
}]
#[async_trait]
pub trait CanSendPacket<Counterparty>:
    HasPacketHeaderType<Counterparty>
    + HasPayloadType<Counterparty>
    + HasPacketType<Counterparty>
    + HasAsyncErrorType
{
    async fn send_packet(
        &mut self,
        packet_header: &Self::PacketHeader,
        payloads: Vec<Self::Payload>,
    ) -> Result<Self::Packet, Self::Error>;
}
