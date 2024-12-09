use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

#[cgp_component {
  provider: PacketBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildPacket<Counterparty>:
    HasPacketType<Counterparty>
    + HasPacketHeaderType<Counterparty>
    + HasPacketNonceType<Counterparty>
    + HasPayloadType<Counterparty>
    + HasErrorType
{
    fn build_packet(
        packet_header: &Self::PacketHeader,
        nonce: Self::PacketNonce,
        payloads: Vec<Self::Payload>,
    ) -> Result<Self::Packet, Self::Error>;
}
