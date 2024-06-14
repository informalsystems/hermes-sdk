use cgp_core::prelude::*;

use crate::chain::traits::types::height::HasHeightType;

#[derive_component(CommitmentProofTypeComponent, ProvideCommitmentProofType<Chain>)]
pub trait HasCommitmentProofType: Async {
    type CommitmentProof: Async;
}

#[derive_component(CommitmentProofHeightGetterComponent, CommitmentProofHeightGetter<Chain>)]
pub trait HasCommitmentProofHeight: HasCommitmentProofType + HasHeightType {
    fn commitment_proof_height(proof: &Self::CommitmentProof) -> &Self::Height;
}

#[derive_component(CommitmentProofBytesGetterComponent, CommitmentProofBytesGetter<Chain>)]
pub trait HasCommitmentProofBytes: HasCommitmentProofType {
    fn commitment_proof_bytes(proof: &Self::CommitmentProof) -> &[u8];
}

pub struct ViaCommitmentProof;
