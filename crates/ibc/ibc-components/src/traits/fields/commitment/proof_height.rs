use hermes_chain_type_components::traits::HasHeightType;
use hermes_prelude::*;

use crate::traits::types::commitment::proof::HasCommitmentProofType;

#[cgp_component {
  provider: CommitmentProofHeightGetter,
  context: Chain,
}]
pub trait HasCommitmentProofHeight<Tag>: HasCommitmentProofType<Tag> + HasHeightType {
    fn commitment_proof_height(proof: &Self::CommitmentProof) -> &Self::Height;
}
