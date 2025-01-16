use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: CommitmentProofTypeComponent,
  provider: ProvideCommitmentProofType,
  context: Chain,
}]
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
