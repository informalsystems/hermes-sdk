use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::payload::HasPacketPayloadHeaderType;

#[derive_component(PacketAckHandlerComponent, PacketPayloadHandler<Chain>)]
#[async_trait]
pub trait CanHandlePacketAck<App, Counterparty>:
    HasErrorType
    + HasPacketHeaderType<Counterparty>
    + HasPacketPayloadHeaderType<Counterparty>
    + HasPacketDataType<App, Counterparty>
where
    Counterparty: HasPacketAckType<App, Self>,
{
    async fn handle_incoming_packet(
        &self,
        packet_header: &Self::PacketHeader,
        payload_header: &Self::PacketPayloadHeader,
        ack: &Counterparty::PacketAck,
    ) -> Result<(), Self::Error>;
}
