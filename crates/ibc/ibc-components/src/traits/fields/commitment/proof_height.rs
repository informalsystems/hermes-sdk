use cgp::prelude::*;

use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::height::HasHeightType;

#[derive_component(CommitmentProofHeightGetterComponent, CommitmentProofHeightGetter<Chain>)]
pub trait HasCommitmentProofHeight: HasCommitmentProofType + HasHeightType {
    fn commitment_proof_height(proof: &Self::CommitmentProof) -> &Self::Height;
}
