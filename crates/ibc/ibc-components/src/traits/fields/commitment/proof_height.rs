use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;

use crate::traits::types::commitment::proof::HasCommitmentProofType;

#[derive_component(CommitmentProofHeightGetterComponent, CommitmentProofHeightGetter<Chain>)]
pub trait HasCommitmentProofHeight<Tag>: HasCommitmentProofType<Tag> + HasHeightType {
    fn commitment_proof_height(proof: &Self::CommitmentProof) -> &Self::Height;
}
