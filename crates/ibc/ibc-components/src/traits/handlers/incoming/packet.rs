use cgp::prelude::*;

use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::tags::commitment::send::SendPacket;

#[derive_component(IncomingPacketHandlerComponent, IncomingPacketHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacket<Counterparty>: Sized + Async + HasErrorType
where
    Counterparty: HasCommitmentProofType<SendPacket> + HasPacketType<Self>,
{
    async fn handle_incoming_packet(
        &mut self,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Self::Error>;
}
