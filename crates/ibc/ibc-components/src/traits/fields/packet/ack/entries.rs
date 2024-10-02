use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::error_ack::HasPacketErrorAckType;
use crate::traits::types::payload::ack::HasPayloadAckType;

#[derive_component(PacketAckEntriesGetterComponent, PacketAckEntriesGetter<Chain>)]
pub trait HasPacketAckEntries<Counterparty, App>:
    HasPacketAckType<Counterparty>
    + HasPayloadAckType<Counterparty, App>
    + HasPacketErrorAckType<Counterparty>
{
    fn packet_ack_entries(
        ack: &Self::PacketAck,
    ) -> Result<&[Self::PayloadAck], &Self::PacketErrorAck>;
}
