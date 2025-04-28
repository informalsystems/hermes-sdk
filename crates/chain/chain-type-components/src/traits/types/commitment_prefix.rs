use hermes_prelude::*;

#[cgp_component {
  name: CommitmentPrefixTypeComponent,
  provider: ProvideCommitmentPrefixType,
  context: Chain,
}]
pub trait HasCommitmentPrefixType: Async {
    type CommitmentPrefix: Async;
}
