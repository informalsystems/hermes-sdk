use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::commitment::value::HasCommitmentValueType;

#[cgp_component {
  provider: CommitmentVerifier,
  context: Chain,
}]
pub trait CanVerifyCommitment<Counterparty, Tag>:
    HasAsyncErrorType
    + HasCommitmentPathType<Tag>
    + HasCommitmentValueType<Tag>
    + HasCommitmentProofType<Tag>
    + HasChannelIdType<Counterparty>
    + HasConsensusStateType<Counterparty>
{
    fn verify_commitment(
        consensus_state: &Self::ConsensusState,
        commitment_path: &Self::CommitmentPath,
        commitment_value: &Self::CommitmentValue,
        proof: &Self::CommitmentProof,
    ) -> Result<(), Self::Error>;
}
