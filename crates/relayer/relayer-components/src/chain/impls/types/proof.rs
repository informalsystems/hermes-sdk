use alloc::vec::Vec;

use cgp_core::Async;

use crate::chain::traits::types::proof::ProvideCommitmentProofType;

pub struct ProvideCommitmentProofBytes;

impl<Chain> ProvideCommitmentProofType<Chain> for ProvideCommitmentProofBytes
where
    Chain: Async,
{
    type CommitmentProof = Vec<u8>;
}
