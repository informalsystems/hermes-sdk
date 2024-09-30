use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(SendPacketCommitmentPathBuilderComponent, SendPacketCommitmentPathBuilder<Chain>)]
pub trait CanBuildSendPacketCommitmentPath<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasCommitmentValueType
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_send_packet_commitment_path(
        packet_header: &Self::PacketHeader,
    ) -> Self::CommitmentValue;
}
