use cgp::prelude::*;

#[derive_component(CommitmentPrefixTypeComponent, ProvideCommitmentPrefixType<Chain>)]
pub trait HasCommitmentPrefixType: Async {
    type CommitmentPrefix: Async;
}
