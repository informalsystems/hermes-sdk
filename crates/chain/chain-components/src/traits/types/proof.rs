use cgp::prelude::*;
pub use hermes_chain_type_components::traits::*;

use crate::traits::HasHeightType;

#[cgp_component {
  provider: CommitmentProofHeightGetter,
  context: Chain,
}]
pub trait HasCommitmentProofHeight: HasCommitmentProofType + HasHeightType {
    fn commitment_proof_height(proof: &Self::CommitmentProof) -> &Self::Height;
}

#[cgp_component {
  provider: CommitmentProofBytesGetter,
  context: Chain,
}]
pub trait HasCommitmentProofBytes: HasCommitmentProofType {
    fn commitment_proof_bytes(proof: &Self::CommitmentProof) -> &[u8];
}

pub struct ViaCommitmentProof;
