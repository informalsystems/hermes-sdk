use cgp::prelude::*;

use crate::traits::types::height::HasHeightType;

pub use hermes_chain_type_components::traits::commitment_proof::*;

#[derive_component(CommitmentProofHeightGetterComponent, CommitmentProofHeightGetter<Chain>)]
pub trait HasCommitmentProofHeight: HasCommitmentProofType + HasHeightType {
    fn commitment_proof_height(proof: &Self::CommitmentProof) -> &Self::Height;
}

#[derive_component(CommitmentProofBytesGetterComponent, CommitmentProofBytesGetter<Chain>)]
pub trait HasCommitmentProofBytes: HasCommitmentProofType {
    fn commitment_proof_bytes(proof: &Self::CommitmentProof) -> &[u8];
}

pub struct ViaCommitmentProof;
