use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_type]
pub trait HasCommitmentProofType: Async {
    type CommitmentProof: Async;
}
