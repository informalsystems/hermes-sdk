use cgp::prelude::*;

use crate::traits::types::commitment::value::HasCommitmentValueType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(PacketCommitmentValueBuilderComponent, PacketCommitmentValueBuilder<Chain>)]
pub trait CanBuildPacketCommitmentValue<Counterparty>:
    HasPacketType<Counterparty> + HasCommitmentValueType
{
    // Note: this may be called by the counterparty chain, thus the lack of access to &self.
    fn build_packet_commitment_value(packet: &Self::Packet) -> Self::CommitmentValue;
}
