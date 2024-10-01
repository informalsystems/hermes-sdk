use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(RawPacketAckEntriesHandlerComponent, RawPacketAckEntriesHandler<Chain>)]
#[async_trait]
pub trait CanHandleRawPacketAckEntries<Counterparty, App>:
    HasErrorType + HasPacketHeaderType<Counterparty> + HasPacketEntryHeaderType<Counterparty>
where
    Counterparty: HasPacketAckType<Self, App>,
{
    async fn handle_raw_packet_ack_entries(
        &self,
        header: &Self::PacketHeader,
        ack_entries: &[(Self::PacketEntryHeader, Counterparty::PacketAck)],
    ) -> Result<(), Self::Error>;
}
