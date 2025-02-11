use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::proof::ProvideCommitmentProofType;
use ibc::core::commitment_types::merkle::MerkleProof;

pub struct ProvideMerkleProofType;

#[cgp_provider(CommitmentProofTypeComponent)]
impl<Chain> ProvideCommitmentProofType<Chain> for ProvideMerkleProofType
where
    Chain: Async,
{
    type CommitmentProof = MerkleProof;
}
