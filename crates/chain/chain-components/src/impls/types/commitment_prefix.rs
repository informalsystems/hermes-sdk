use alloc::vec::Vec;

use cgp::core::Async;

use crate::traits::commitment_prefix::ProvideCommitmentPrefixType;

pub struct ProvideCommitmentPrefixBytes;

impl<Chain> ProvideCommitmentPrefixType<Chain> for ProvideCommitmentPrefixBytes
where
    Chain: Async,
{
    type CommitmentPrefix = Vec<u8>;
}
