use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_proof::CommitmentProofTypeComponent;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::proof::{
    CommitmentProofBytesGetter, CommitmentProofBytesGetterComponent, CommitmentProofHeightGetter,
    CommitmentProofHeightGetterComponent, HasCommitmentProofType, ProvideCommitmentProofType,
};
use ibc::core::client::types::Height;
use ibc::core::commitment_types::merkle::MerkleProof;

pub struct CosmosCommitmentProof {
    pub merkle_proof: MerkleProof,
    pub proof_bytes: Vec<u8>,
    pub proof_height: Height,
}

pub struct ProvideCosmosCommitmentProof;

#[cgp_provider(CommitmentProofTypeComponent)]
impl<Chain> ProvideCommitmentProofType<Chain> for ProvideCosmosCommitmentProof
where
    Chain: Async,
{
    type CommitmentProof = CosmosCommitmentProof;
}

#[cgp_provider(CommitmentProofHeightGetterComponent)]
impl<Chain> CommitmentProofHeightGetter<Chain> for ProvideCosmosCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
        + HasHeightType<Height = Height>,
{
    fn commitment_proof_height(proof: &CosmosCommitmentProof) -> &Height {
        &proof.proof_height
    }
}

#[cgp_provider(CommitmentProofBytesGetterComponent)]
impl<Chain> CommitmentProofBytesGetter<Chain> for ProvideCosmosCommitmentProof
where
    Chain: HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>,
{
    fn commitment_proof_bytes(proof: &CosmosCommitmentProof) -> &[u8] {
        &proof.proof_bytes
    }
}
