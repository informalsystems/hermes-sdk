use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetter, CommitmentProofHeightGetter, HasCommitmentProofType,
    ProvideCommitmentProofType,
};
use jmt::proof::SparseMerkleProof;
use sha2::Sha256;

use crate::types::height::RollupHeight;

pub type JellyfishMerkleProof = SparseMerkleProof<Sha256>;

pub struct SovereignCommitmentProof {
    pub merkle_proof: JellyfishMerkleProof,
    pub proof_bytes: Vec<u8>,
    pub proof_height: RollupHeight,
}

pub struct ProvideSovereignCommitmentProof;

impl<Chain> ProvideCommitmentProofType<Chain> for ProvideSovereignCommitmentProof
where
    Chain: Async,
{
    type CommitmentProof = SovereignCommitmentProof;
}

impl<Chain> CommitmentProofHeightGetter<Chain> for ProvideSovereignCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = SovereignCommitmentProof>
        + HasHeightType<Height = RollupHeight>,
{
    fn commitment_proof_height(proof: &SovereignCommitmentProof) -> &RollupHeight {
        &proof.proof_height
    }
}

impl<Chain> CommitmentProofBytesGetter<Chain> for ProvideSovereignCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = SovereignCommitmentProof>,
{
    fn commitment_proof_bytes(proof: &SovereignCommitmentProof) -> &[u8] {
        &proof.proof_bytes
    }
}
