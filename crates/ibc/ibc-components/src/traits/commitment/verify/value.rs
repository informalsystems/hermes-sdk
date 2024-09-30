use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::commitment::value::HasCommitmentValueType;

pub trait CanVerifyValueCommitment<Counterparty>:
    HasErrorType
    + HasCommitmentPathType
    + HasCommitmentValueType
    + HasCommitmentProofType
    + HasClientIdType<Counterparty>
    + HasConsensusStateType<Counterparty>
{
    fn verify_value_commitment(
        consensus_state: &Self::ConsensusState,
        commitment_path: &Self::CommitmentPath,
        commitment_value: &Self::CommitmentValue,
        proof: &Self::CommitmentProof,
    ) -> Result<(), Self::Error>;
}
