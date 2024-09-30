use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;

#[derive_component(IncomingPacketHandlerComponent, IncomingPacketHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacket<Counterparty>:
    HasErrorType + HasPacketRawAckType<Counterparty>
where
    Counterparty: HasPacketType<Self>,
{
    async fn handle_incoming_packet(
        &self,
        packet: &Counterparty::Packet,
    ) -> Result<Vec<Self::PacketRawAck>, Self::Error>;
}
