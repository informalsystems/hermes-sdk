use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(IncomingPacketEntryHandlerComponent, IncomingPacketEntryHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacketEntry<App, Counterparty>:
    HasErrorType + HasPacketDataType<App, Counterparty> + HasPacketAckType<App, Counterparty>
where
    Counterparty: HasPacketHeaderType<Self> + HasPacketEntryHeaderType<Self>,
{
    async fn handle_incoming_packet_entry(
        &self,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PacketEntryHeader,
        entry_data: &Self::PacketData,
    ) -> Result<Self::PacketAck, Self::Error>;
}
