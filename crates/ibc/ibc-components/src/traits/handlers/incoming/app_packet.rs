use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::payload::HasPacketPayloadHeaderType;

#[derive_component(IncomingRawPacketPayloadHandlerComponent, IncomingRawPacketPayloadHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacket<App, Counterparty>:
    HasErrorType + HasPacketDataType<App, Counterparty> + HasPacketAckType<App, Counterparty>
where
    Counterparty: HasPacketHeaderType<Self> + HasPacketPayloadHeaderType<Self>,
{
    async fn handle_incoming_packet(
        &self,
        header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PacketPayloadHeader,
        payload_data: &Self::PacketData,
    ) -> Result<Self::PacketAck, Self::Error>;
}
