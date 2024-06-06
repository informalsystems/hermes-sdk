use cgp_core::prelude::*;

#[derive_component(CommitmentProofTypeComponent, ProvideCommitmentProofType<Chain>)]
pub trait HasCommitmentProofType: Async {
    type CommitmentProof: Async;
}

pub struct ViaCommitmentProof;
