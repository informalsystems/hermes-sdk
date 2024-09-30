use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;

#[derive_component(AckPacketCommitmentValueBuilderComponent, AckPacketCommitmentValueBuilder<Chain>)]
pub trait CanBuildAckPacketCommitmentValue<Counterparty>:
    HasPacketRawAckType<Counterparty> + HasCommitmentValueType
where
    Counterparty: HasPacketHeaderType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_ack_packet_commitment_value(
        packet_header: &Counterparty::PacketHeader,
        acks: &[Self::PacketRawAck],
    ) -> Self::CommitmentValue;
}
