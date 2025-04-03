use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_type]
pub trait HasCommitmentProofType: Async {
    type CommitmentProof: Async;
}
