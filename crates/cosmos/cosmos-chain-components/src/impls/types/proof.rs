use cgp::prelude::*;
use hermes_core::chain_type_components::traits::CommitmentProofTypeProviderComponent;
use hermes_core::relayer_components::chain::traits::CommitmentProofTypeProvider;
use ibc::core::commitment_types::merkle::MerkleProof;

pub struct ProvideMerkleProofType;

#[cgp_provider(CommitmentProofTypeProviderComponent)]
impl<Chain> CommitmentProofTypeProvider<Chain> for ProvideMerkleProofType
where
    Chain: Async,
{
    type CommitmentProof = MerkleProof;
}
