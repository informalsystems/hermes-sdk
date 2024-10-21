use cgp::prelude::*;

#[derive_component(CommitmentValueTypeComponent, ProvideCommitmentValueType<Chain>)]
pub trait HasCommitmentValueType<Tag>: Async {
    type CommitmentValue: Async;
}
