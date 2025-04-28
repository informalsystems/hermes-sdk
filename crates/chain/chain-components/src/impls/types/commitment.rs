use alloc::vec::Vec;

use hermes_prelude::*;

use crate::traits::{PacketCommitmentTypeComponent, ProvidePacketCommitmentType};

pub struct ProvideBytesPacketCommitment;

#[cgp_provider(PacketCommitmentTypeComponent)]
impl<Chain, Counterparty> ProvidePacketCommitmentType<Chain, Counterparty>
    for ProvideBytesPacketCommitment
where
    Chain: Async,
{
    type PacketCommitment = Vec<u8>;
}
