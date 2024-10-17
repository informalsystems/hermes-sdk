use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(CommitmentProofTypeComponent, ProvideCommitmentProofType<Chain>)]
pub trait HasCommitmentProofType<Tag>: Async {
    type CommitmentProof: Async;
}

impl<Chain, Tag, Provider, CommitmentProof> ProvideCommitmentProofType<Chain, Tag>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, CommitmentProofTypeComponent, Type = CommitmentProof>,
    CommitmentProof: Async,
{
    type CommitmentProof = CommitmentProof;
}