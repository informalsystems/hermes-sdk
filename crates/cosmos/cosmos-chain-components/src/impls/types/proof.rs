use cgp::core::Async;
use hermes_relayer_components::chain::traits::types::proof::ProvideCommitmentProofType;
use ibc::core::commitment_types::merkle::MerkleProof;

pub struct ProvideMerkleProofType;

impl<Chain> ProvideCommitmentProofType<Chain> for ProvideMerkleProofType
where
    Chain: Async,
{
    type CommitmentProof = MerkleProof;
}
