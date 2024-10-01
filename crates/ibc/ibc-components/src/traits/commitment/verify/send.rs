use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::types::packet::header::HasPacketHeaderType;

pub trait CanVerifySendPacketCommitment<Counterparty>:
    HasErrorType
    + HasCommitmentProofType
    + HasPacketHeaderType<Counterparty>
    + HasConsensusStateType<Counterparty>
    + HasChannelIdType<Counterparty>
{
    // Note: this will be called by the counterparty chain, thus the lack of access to &self.
    fn verify_send_packet_commitment(
        consensus_state: &Self::ConsensusState,
        packet_header: &Self::PacketHeader,
        proof: &Self::CommitmentProof,
    ) -> Result<(), Self::Error>;
}
