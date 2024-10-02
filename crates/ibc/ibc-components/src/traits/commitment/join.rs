use cgp::prelude::*;

use crate::traits::types::commitment::path::HasCommitmentPathType;

#[derive_component(CommitmentPathJoinerComponent, CommitmentPathJoiner<Chain>)]
pub trait CanJoinCommitmentPaths: HasCommitmentPathType + HasErrorType {
    fn join_commitment_paths(
        path_a: &Self::CommitmentPath,
        path_b: &Self::CommitmentPath,
    ) -> Result<Self::CommitmentPath, Self::Error>;
}
