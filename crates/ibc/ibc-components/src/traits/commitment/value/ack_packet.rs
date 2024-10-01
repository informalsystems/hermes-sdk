use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(AckPacketCommitmentValueBuilderComponent, AckPacketCommitmentValueBuilder<Chain>)]
pub trait CanBuildAckPacketCommitmentValue<Counterparty>:
    HasPacketAckType<Counterparty> + HasCommitmentValueType + HasErrorType
where
    Counterparty: HasPacketHeaderType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_ack_packet_commitment_value(
        packet_header: &Counterparty::PacketHeader,
        ack: &Self::PacketAck,
    ) -> Result<Self::CommitmentValue, Self::Error>;
}
