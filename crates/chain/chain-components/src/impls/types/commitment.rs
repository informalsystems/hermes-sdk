use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::packets::receive::{
    PacketCommitmentTypeComponent, ProvidePacketCommitmentType,
};

pub struct ProvideBytesPacketCommitment;

#[cgp_provider(PacketCommitmentTypeComponent)]
impl<Chain, Counterparty> ProvidePacketCommitmentType<Chain, Counterparty>
    for ProvideBytesPacketCommitment
where
    Chain: Async,
{
    type PacketCommitment = Vec<u8>;
}
