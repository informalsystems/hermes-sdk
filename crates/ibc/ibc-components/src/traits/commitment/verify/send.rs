use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::ibc::client_state::HasClientStateType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::types::packet::header::HasPacketHeaderType;

pub trait CanVerifySendPacketCommitment<Counterparty>:
    HasErrorType
    + HasPacketHeaderType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasConsensusStateType<Counterparty>
    + HasClientIdType<Counterparty>
{
    fn verify_send_packet_commitment(
        client_id: &Self::ClientId,
        client_state: &Self::ClientState,
        consensus_state: &Self::ConsensusState,
        packet_header: &Self::PacketHeader,
    ) -> Result<(), Self::Error>;
}
