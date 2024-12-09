use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: CommitmentProofTypeComponent,
  provider: ProvideCommitmentProofType,
  context: Chain,
}]
pub trait HasCommitmentProofType: Async {
    type CommitmentProof: Async;
}

impl<Chain, Provider, CommitmentProof> ProvideCommitmentProofType<Chain> for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, CommitmentProofTypeComponent, Type = CommitmentProof>,
    CommitmentProof: Async,
{
    type CommitmentProof = CommitmentProof;
}
