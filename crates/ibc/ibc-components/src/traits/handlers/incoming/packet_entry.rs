use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::entry_ack::HasPacketEntryAckType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(IncomingPacketEntryHandlerComponent, IncomingPacketEntryHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacketEntry<Counterparty, App>:
    HasErrorType + HasPacketEntryAckType<Counterparty, App>
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPacketEntryHeaderType<Self> + HasPacketDataType<Self, App>,
{
    async fn handle_incoming_packet_entry(
        &self,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PacketEntryHeader,
        entry_data: &Counterparty::PacketData,
    ) -> Result<Self::PacketEntryAck, Self::Error>;
}
