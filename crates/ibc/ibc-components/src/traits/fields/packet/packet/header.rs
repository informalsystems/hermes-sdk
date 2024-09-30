use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(PacketHeaderGetterComponent, PacketHeaderGetter<Chain>)]
pub trait HasPacketHeader<Counterparty>:
    HasPacketType<Counterparty> + HasPacketHeaderType<Counterparty>
{
    fn packet_header(packet: &Self::Packet) -> &Self::PacketHeader;
}
