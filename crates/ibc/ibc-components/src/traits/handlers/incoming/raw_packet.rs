use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::payload::HasPacketPayloadHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;

#[derive_component(IncomingRawPacketHandlerComponent, IncomingRawPacketHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingRawPacket<Counterparty>:
    HasErrorType + HasPacketRawAckType<Counterparty>
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPacketPayloadHeaderType<Self> + HasPacketRawDataType<Self>,
{
    async fn handle_incoming_raw_packet(
        &self,
        header: &Counterparty::PacketHeader,
        payloads: &[(
            Counterparty::PacketPayloadHeader,
            Counterparty::PacketRawData,
        )],
    ) -> Result<Vec<Self::PacketRawAck>, Self::Error>;
}
