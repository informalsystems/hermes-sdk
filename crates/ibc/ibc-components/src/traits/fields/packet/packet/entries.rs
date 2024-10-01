use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(PacketEntriesGetterComponent, PacketEntriesGetter<Chain>)]
pub trait HasPacketEntries<Counterparty, App>:
    HasPacketType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketDataType<Counterparty, App>
{
    fn packet_entries(packet: &Self::Packet) -> &[(Self::PacketEntryHeader, Self::PacketData)];
}
