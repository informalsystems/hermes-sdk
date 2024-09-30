use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::payload::HasPacketPayloadHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;

#[derive_component(RawPacketPayloadAckHandlerComponent, RawPacketPayloadAckHandler<Chain>)]
#[async_trait]
pub trait CanHandleRawPacketPayloadAck<Counterparty>:
    HasErrorType + HasPacketHeaderType<Counterparty> + HasPacketPayloadHeaderType<Counterparty>
where
    Counterparty: HasPacketRawAckType<Self>,
{
    async fn handle_raw_packet_payload_ack(
        &self,
        header: &Self::PacketHeader,
        payload_header: &Self::PacketPayloadHeader,
        ack: &Counterparty::PacketRawAck,
    ) -> Result<(), Self::Error>;
}
