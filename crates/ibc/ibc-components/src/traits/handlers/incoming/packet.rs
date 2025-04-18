use cgp::prelude::*;

use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::tags::commitment::send::SendPacket;

#[cgp_component {
  provider: IncomingPacketHandler,
  context: Chain,
}]
#[async_trait]
pub trait CanHandleIncomingPacket<Counterparty>: Sized + Async + HasAsyncErrorType
where
    Counterparty: HasCommitmentProofType<SendPacket> + HasPacketType<Self>,
{
    async fn handle_incoming_packet(
        &mut self,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Self::Error>;
}
