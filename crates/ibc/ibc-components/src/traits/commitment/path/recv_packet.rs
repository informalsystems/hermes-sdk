use cgp::prelude::*;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(RecvPacketCommitmentPathBuilderComponent, RecvPacketCommitmentPathBuilder<Chain>)]
pub trait CanBuildRecvPacketCommitmentPath<Counterparty>:
    HasCommitmentPathType + HasErrorType
where
    Counterparty: HasPacketHeaderType<Self>,
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_recv_packet_commitment_path(
        packet_header: &Counterparty::PacketHeader,
    ) -> Result<Self::CommitmentPath, Self::Error>;
}
