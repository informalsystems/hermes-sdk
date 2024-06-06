use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::proof::ProvideCommitmentProofType;
use jmt::proof::SparseMerkleProof;
use sha2::Sha256;

pub type JellyfishMerkleProof = SparseMerkleProof<Sha256>;

pub struct ProvideJellyfishMerkleProof;

impl<Chain> ProvideCommitmentProofType<Chain> for ProvideJellyfishMerkleProof
where
    Chain: Async,
{
    type CommitmentProof = JellyfishMerkleProof;
}
