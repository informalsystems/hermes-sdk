use alloc::vec::Vec;

use cgp::core::Async;

use crate::chain::traits::types::packets::receive::ProvidePacketCommitmentType;

pub struct ProvideBytesPacketCommitment;

impl<Chain, Counterparty> ProvidePacketCommitmentType<Chain, Counterparty>
    for ProvideBytesPacketCommitment
where
    Chain: Async,
{
    type PacketCommitment = Vec<u8>;
}
