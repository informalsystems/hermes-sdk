use cgp::prelude::*;

#[derive_component(CommitmentPathTypeComponent, ProvideCommitmentPathType<Chain>)]
pub trait HasCommitmentPathType: Async {
    type CommitmentPath: Async;
}
