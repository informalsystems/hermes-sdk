use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;

#[derive_component(IncomingPacketHandlerComponent, IncomingPacketHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacket<App, Counterparty>:
    HasErrorType + HasPacketAckType<App, Counterparty>
where
    Counterparty: HasCommitmentProofType + HasPacketType<Self>,
{
    async fn handle_incoming_packet(
        &self,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Vec<Self::PacketAck>, Self::Error>;
}
