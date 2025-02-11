use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_proof::CommitmentProofTypeComponent;

use crate::traits::types::proof::ProvideCommitmentProofType;

pub struct ProvideCommitmentProofBytes;

#[cgp_provider(CommitmentProofTypeComponent)]
impl<Chain> ProvideCommitmentProofType<Chain> for ProvideCommitmentProofBytes
where
    Chain: Async,
{
    type CommitmentProof = Vec<u8>;
}
