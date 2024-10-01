use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::entry_ack::HasPacketEntryAckType;
use crate::traits::types::packet::error_ack::HasPacketErrorAckType;

#[derive_component(PacketAckEntriesGetterComponent, PacketAckEntriesGetter<Chain>)]
pub trait HasPacketAckEntries<Counterparty, App>:
    HasPacketAckType<Counterparty>
    + HasPacketEntryAckType<Counterparty, App>
    + HasPacketErrorAckType<Counterparty>
{
    fn packet_ack_entries(
        ack: &Self::PacketAck,
    ) -> Result<&[Self::PacketEntryAck], &Self::PacketErrorAck>;
}
