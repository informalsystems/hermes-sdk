use cgp::prelude::*;

use crate::traits::types::commitment::path::HasCommitmentPathType;

#[derive_component(CommitmentPathJoinerComponent, CommitmentPathJoiner<Chain>)]
pub trait CanJoinCommitmentPaths: HasCommitmentPathType {
    fn join_commitment_paths(
        path_a: &Self::CommitmentPath,
        path_b: &Self::CommitmentPath,
    ) -> Self::CommitmentPath;
}
