use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::error_ack::HasPacketErrorAckType;
use crate::traits::types::payload::ack::HasPayloadAckType;

#[derive_component(PayloadAcksGetterComponent, PayloadAcksGetter<Chain>)]
pub trait HasPayloadAcks<Counterparty, App>:
    HasPacketAckType<Counterparty>
    + HasPayloadAckType<Counterparty, App>
    + HasPacketErrorAckType<Counterparty>
{
    fn packet_payload_acks(
        ack: &Self::PacketAck,
    ) -> Result<&[Self::PayloadAck], &Self::PacketErrorAck>;
}
