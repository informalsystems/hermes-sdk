use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::client_state::HasClientStateType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;

pub trait CanVerifyAckPacketCommitment<Counterparty>:
    HasErrorType
    + HasPacketHeaderType<Counterparty>
    + HasPacketRawAckType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasConsensusStateType<Counterparty>
    + HasClientIdType<Counterparty>
{
    fn verify_ack_packet_commitment(
        client_id: &Self::ClientId,
        client_state: &Self::ClientState,
        consensus_state: &Self::ConsensusState,
        packet_header: &Self::PacketHeader,
        packet_acks: &[&Self::PacketRawAck],
    ) -> Result<(), Self::Error>;
}
