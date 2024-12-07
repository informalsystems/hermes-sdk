use cgp::prelude::*;

#[derive_component(CommitmentValueTypeComponent, ProvideCommitmentValueType<Chain>)]
pub trait HasCommitmentValueType<Tag>: Sized + Async {
    type CommitmentValue: Async;
}
