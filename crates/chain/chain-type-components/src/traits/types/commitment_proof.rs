use cgp::prelude::*;

#[derive_component(CommitmentProofTypeComponent, ProvideCommitmentProofType<Chain>)]
pub trait HasCommitmentProofType: Async {
    type CommitmentProof: Async;
}
