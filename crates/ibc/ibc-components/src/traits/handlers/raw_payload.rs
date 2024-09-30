use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::payload::HasPacketPayloadHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;

#[derive_component(IncomingRawPacketPayloadHandlerComponent, IncomingRawPacketPayloadHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingRawPacketPayload<Counterparty>:
    HasErrorType + HasPacketRawAckType<Counterparty>
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPacketPayloadHeaderType<Self> + HasPacketRawDataType<Self>,
{
    async fn handle_incoming_raw_packet_payload(
        &self,
        header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PacketPayloadHeader,
        payload_data: &Counterparty::PacketRawData,
    ) -> Result<Self::PacketRawAck, Self::Error>;
}
