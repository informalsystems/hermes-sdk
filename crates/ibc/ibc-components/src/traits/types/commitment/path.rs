use cgp::prelude::*;

#[derive_component(CommitmentPathTypeComponent, ProvideCommitmentPathType<Chain>)]
pub trait HasCommitmentPathType<Tag>: Async {
    type CommitmentPath: Async;
}
