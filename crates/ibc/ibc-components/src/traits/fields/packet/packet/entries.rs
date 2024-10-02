use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PacketEntriesGetterComponent, PacketEntriesGetter<Chain>)]
pub trait HasPacketEntries<Counterparty, App>:
    HasPacketType<Counterparty>
    + HasPayloadHeaderType<Counterparty>
    + HasPacketDataType<Counterparty, App>
{
    fn packet_entries(packet: &Self::Packet) -> &[(Self::PayloadHeader, Self::PacketData)];
}
