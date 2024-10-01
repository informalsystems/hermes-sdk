use alloc::vec::Vec;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(IncomingPacketHandlerComponent, IncomingPacketHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacket<Counterparty, App>:
    HasErrorType + HasPacketAckType<Counterparty, App>
where
    Counterparty: HasCommitmentProofType + HasPacketType<Self>,
{
    async fn handle_incoming_packet(
        &self,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Vec<Self::PacketAck>, Self::Error>;
}
