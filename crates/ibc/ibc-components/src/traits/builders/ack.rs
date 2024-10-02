use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::payload::ack::HasPayloadAckType;

#[derive_component(PacketAckFromPayloadsBuilderComponent, PacketAckFromPayloadsBuilder<Chain>)]
pub trait CanBuildPacketAckFromPayloads<Counterparty, App>:
    HasPacketAckType<Counterparty> + HasPayloadAckType<Counterparty, App>
{
    fn build_packet_ack_from_payload_acks(acks: Vec<Self::PayloadAck>) -> Self::PacketAck;
}
