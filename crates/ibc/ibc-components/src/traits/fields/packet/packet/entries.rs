use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PacketEntriesGetterComponent, PacketEntriesGetter<Chain>)]
pub trait HasPacketEntries<Counterparty, App>:
    HasPacketType<Counterparty>
    + HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
{
    fn packet_entries(packet: &Self::Packet) -> &[(Self::PayloadHeader, Self::PayloadData)];
}
