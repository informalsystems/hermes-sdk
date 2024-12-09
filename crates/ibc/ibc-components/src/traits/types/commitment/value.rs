use cgp::prelude::*;

#[cgp_component {
  name: CommitmentValueTypeComponent,
  provider: ProvideCommitmentValueType,
  context: Chain,
}]
pub trait HasCommitmentValueType<Tag>: Sized + Async {
    type CommitmentValue: Async;
}
