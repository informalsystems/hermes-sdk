use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::payload::ack::HasPayloadAckType;

#[derive_component(PacketAckFromEntriesBuilderComponent, PacketAckFromEntriesBuilder<Chain>)]
pub trait CanBuildPacketAckFromEntries<Counterparty, App>:
    HasPacketAckType<Counterparty> + HasPayloadAckType<Counterparty, App>
{
    fn build_packet_ack_from_entries(entries: Vec<Self::PayloadAck>) -> Self::PacketAck;
}
