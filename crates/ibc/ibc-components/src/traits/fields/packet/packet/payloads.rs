use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PacketPayloadsGetterComponent, PacketPayloadsGetter<Chain>)]
pub trait HasPacketPayloads<Counterparty, App>:
    HasPacketType<Counterparty>
    + HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
{
    fn packet_payloads(packet: &Self::Packet) -> &[(Self::PayloadHeader, Self::PayloadData)];
}
