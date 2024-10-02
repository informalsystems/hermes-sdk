use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::ack::HasPayloadAckType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PacketAckHandlerComponent, PacketPayloadHandler<Chain>)]
#[async_trait]
pub trait CanHandlePacketAck<Counterparty, App>:
    HasErrorType
    + HasPacketHeaderType<Counterparty>
    + HasPayloadHeaderType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
where
    Counterparty: HasPayloadAckType<Self, App>,
{
    async fn handle_packet_ack(
        &self,
        packet_header: &Self::PacketHeader,
        entry_header: &Self::PayloadHeader,
        entry_ack: &Counterparty::PayloadAck,
    ) -> Result<(), Self::Error>;
}
