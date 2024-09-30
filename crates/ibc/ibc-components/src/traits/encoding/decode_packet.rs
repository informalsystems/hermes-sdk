use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;

pub trait CanDecodePacket<Counterparty>: HasErrorType + HasPacketType<Counterparty> {
    fn decode_packet(&self, bytes: &[u8]) -> Result<Self::Packet, Self::Error>;
}
