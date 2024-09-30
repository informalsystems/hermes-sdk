use cgp::prelude::*;

use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;

#[derive_component(PacketEntriesGetterComponent, PacketEntriesGetter<Chain>)]
pub trait HasPacketEntries<Counterparty>:
    HasPacketType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketRawDataType<Counterparty>
{
    fn packet_entries(packet: &Self::Packet) -> &[(Self::PacketEntryHeader, Self::PacketRawData)];
}
