use cgp::prelude::*;

use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;

#[derive_component(IncomingRawPacketEntryHandlerComponent, IncomingRawPacketEntryHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingRawPacketEntry<Counterparty>:
    HasErrorType + HasPacketRawAckType<Counterparty>
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPacketEntryHeaderType<Self> + HasPacketRawDataType<Self>,
{
    async fn handle_incoming_raw_packet_entry(
        &self,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PacketEntryHeader,
        entry_data: &Counterparty::PacketRawData,
    ) -> Result<Self::PacketRawAck, Self::Error>;
}
