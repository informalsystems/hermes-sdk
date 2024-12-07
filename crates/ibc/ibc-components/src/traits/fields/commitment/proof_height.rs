use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::commitment::proof::HasCommitmentProofType;

#[cgp_component {
  name: CommitmentProofHeightGetterComponent,
  provider: CommitmentProofHeightGetter,
  context: Chain,
}]
pub trait HasCommitmentProofHeight<Tag>: HasCommitmentProofType<Tag> + HasHeightType {
    fn commitment_proof_height(proof: &Self::CommitmentProof) -> &Self::Height;
}
