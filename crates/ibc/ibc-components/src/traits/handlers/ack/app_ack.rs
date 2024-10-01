use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::entry_ack::HasPacketEntryAckType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketAckHandlerComponent, PacketPayloadHandler<Chain>)]
#[async_trait]
pub trait CanHandlePacketAck<Counterparty, App>:
    HasErrorType
    + HasPacketHeaderType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketDataType<Counterparty, App>
where
    Counterparty: HasPacketEntryAckType<Self, App>,
{
    async fn handle_packet_ack(
        &self,
        packet_header: &Self::PacketHeader,
        entry_header: &Self::PacketEntryHeader,
        entry_ack: &Counterparty::PacketEntryAck,
    ) -> Result<(), Self::Error>;
}
