use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::client_state::HasClientStateType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::header::HasPacketHeaderType;

pub trait CanVerifyAckPacketCommitment<Counterparty, App>:
    HasErrorType
    + HasCommitmentProofType
    + HasPacketHeaderType<Counterparty>
    + HasPacketAckType<Counterparty, App>
    + HasClientStateType<Counterparty>
    + HasConsensusStateType<Counterparty>
    + HasClientIdType<Counterparty>
{
    // Note: this will be called by the counterparty chain, thus the lack of access to &self.
    fn verify_ack_packet_commitment(
        client_state: &Self::ClientState,
        consensus_state: &Self::ConsensusState,
        packet_header: &Self::PacketHeader,
        packet_acks: &[&Self::PacketAck],
        proof: &Self::CommitmentProof,
    ) -> Result<(), Self::Error>;
}
