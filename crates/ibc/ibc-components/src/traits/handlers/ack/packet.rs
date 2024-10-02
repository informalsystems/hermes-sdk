use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketAckHandlerComponent, PacketAckHandler<Chain>)]
#[async_trait]
pub trait CanHandlePacketAck<Counterparty>:
    HasErrorType + HasPacketHeaderType<Counterparty>
where
    Counterparty: HasPacketAckType<Self>,
{
    async fn handle_packet_ack(
        &self,
        header: &Self::PacketHeader,
        ack: &Counterparty::PacketAck,
    ) -> Result<(), Self::Error>;
}
