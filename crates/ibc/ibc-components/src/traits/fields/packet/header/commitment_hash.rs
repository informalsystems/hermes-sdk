use cgp::prelude::*;

use crate::traits::types::packet::commitment_hash::HasPacketCommitmentHashType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketCommitmentHashGetterComponent, PacketCommitmentHashGetter<Chain>)]
pub trait HasPacketCommitmentHash<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasPacketCommitmentHashType<Counterparty>
{
    fn packet_commitment_hash(packet: &Self::PacketHeader) -> &Self::PacketCommitmentHash;
}
