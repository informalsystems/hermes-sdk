use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(PacketEntriesGetterComponent, PacketEntriesGetter<Chain>)]
pub trait HasPacketEntries<App, Counterparty>:
    HasPacketType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketDataType<App, Counterparty>
{
    fn packet_entries(packet: &Self::Packet) -> &[(Self::PacketEntryHeader, Self::PacketData)];
}
