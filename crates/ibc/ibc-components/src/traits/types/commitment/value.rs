use cgp::prelude::*;

#[derive_component(CommitmentValueTypeComponent, ProvideCommitmentValueType<Chain>)]
pub trait HasCommitmentValueType: Async {
    type CommitmentValue: Async;
}
