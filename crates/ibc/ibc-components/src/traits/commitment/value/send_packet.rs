use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;

#[derive_component(PacketCommitmentValueBuilderComponent, PacketCommitmentValueBuilder<Chain>)]
pub trait CanBuildPacketCommitmentValue<Counterparty>:
    HasPacketHeaderType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketRawDataType<Counterparty>
    + HasCommitmentValueType
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_packet_commitment_value(
        packet: &Self::PacketHeader,
        entries: &[(Self::PacketEntryHeader, Self::PacketRawData)],
    ) -> Self::CommitmentValue;
}
