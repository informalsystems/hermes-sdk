use hermes_prelude::*;

#[cgp_component {
  name: CommitmentPathTypeComponent,
  provider: ProvideCommitmentPathType,
  context: Chain,
}]
pub trait HasCommitmentPathType<Tag>: Async {
    type CommitmentPath: Async;
}
