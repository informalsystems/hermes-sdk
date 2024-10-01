use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::entry_ack::HasPacketEntryAckType;

#[derive_component(PacketAckFromEntriesBuilderComponent, PacketAckFromEntriesBuilder<Chain>)]
pub trait CanBuildPacketAckFromEntries<Counterparty, App>:
    HasPacketAckType<Counterparty> + HasPacketEntryAckType<Counterparty, App>
{
    fn build_packet_ack_from_entries(entries: Vec<Self::PacketEntryAck>) -> Self::PacketAck;
}
