use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;

#[derive_component(IncomingRawPacketEntriesHandlerComponent, IncomingRawPacketEntriesHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingRawPacket<Counterparty>:
    HasErrorType + HasPacketRawAckType<Counterparty>
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPacketEntryHeaderType<Self> + HasPacketRawDataType<Self>,
{
    async fn handle_incoming_raw_packet_entries(
        &self,
        header: &Counterparty::PacketHeader,
        entries: &[(Counterparty::PacketEntryHeader, Counterparty::PacketRawData)],
    ) -> Result<Vec<Self::PacketRawAck>, Self::Error>;
}
