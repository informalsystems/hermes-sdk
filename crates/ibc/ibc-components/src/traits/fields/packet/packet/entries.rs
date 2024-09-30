use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::any_app::AnyApp;

#[derive_component(PacketEntriesGetterComponent, PacketEntriesGetter<Chain>)]
pub trait HasPacketEntries<Counterparty>:
    HasPacketType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketDataType<AnyApp, Counterparty>
{
    fn packet_entries(packet: &Self::Packet) -> &[(Self::PacketEntryHeader, Self::PacketData)];
}
