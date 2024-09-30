use cgp::prelude::*;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(AckPacketCommitmentPathBuilderComponent, AckPacketCommitmentPathBuilder<Chain>)]
pub trait CanBuildAckPacketCommitmentPath<Counterparty>: HasCommitmentPathType
where
    Counterparty: HasPacketHeaderType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_ack_packet_commitment_path(
        packet_header: &Counterparty::PacketHeader,
    ) -> Self::CommitmentPath;
}
